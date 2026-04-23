use crate::game::{Action, Board, Game, Status};
use axum::{
    Router,
    extract::{
        State as AxumState,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::any,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

pub struct Lobby {
    pub game: Mutex<Game>,
    pub tx: broadcast::Sender<String>,
}

#[derive(serde::Serialize)]
pub struct ServerMessage {
    board: Board,
    msg: Option<String>,
    err: Option<String>,
}

pub async fn start() {
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(Lobby {
        game: Mutex::new(Game::new()),
        tx,
    });

    let app = Router::new()
        .route("/ws", any(join_game))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Web server live at ws://localhost:3000/ws");
    axum::serve(listener, app).await.unwrap()
}

async fn join_game(
    ws: WebSocketUpgrade,
    AxumState(state): AxumState<Arc<Lobby>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| run_game_session(socket, state))
}

async fn run_game_session(socket: WebSocket, state: Arc<Lobby>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let initial_state = {
        let game = state.game.lock().unwrap();
        let payload = ServerMessage {
            board: *game.board(),
            msg: Some("Game Started!".into()),
            err: None,
        };
        serde_json::to_string(&payload).unwrap()
    };

    let _ = sender.send(Message::Text(initial_state.into())).await;

    let mut broadcaster = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let mut listener = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            match text.parse::<Action>() {
                Ok(Action::Move(idx)) => {
                    let mut game = state.game.lock().unwrap();
                    let result = game.make_move(idx);

                    let message = match result {
                        Ok(Status::Won(p)) => ServerMessage {
                            board: *game.board(),
                            msg: Some(format!("Player {} wins!", p)),
                            err: None,
                        },
                        Ok(Status::Draw) => ServerMessage {
                            board: *game.board(),
                            msg: Some("It's a Draw!".into()),
                            err: None,
                        },
                        Ok(Status::InProgress) => ServerMessage {
                            board: *game.board(),
                            msg: None,
                            err: None,
                        },
                        Err(e) => ServerMessage {
                            board: *game.board(),
                            msg: None,
                            err: Some(format!("Error: {:?}", e)),
                        },
                    };
                    let payload = serde_json::to_string(&message).unwrap();
                    let _ = state.tx.send(payload);
                }
                Ok(Action::Quit) => {
                    let game = state.game.lock().unwrap();
                    let message = ServerMessage {
                        board: *game.board(),
                        msg: Some("A player has left the game.".into()),
                        err: None,
                    };
                    let _ = state.tx.send(serde_json::to_string(&message).unwrap());
                    break;
                }
                Ok(Action::Invalid) => {
                    let game = state.game.lock().unwrap();
                    let message = ServerMessage {
                        board: *game.board(),
                        msg: None,
                        err: Some("Invalid Input.".into()),
                    };
                    let _ = state.tx.send(serde_json::to_string(&message).unwrap());
                    continue;
                }
                Err(e) => {
                    let game = state.game.lock().unwrap();
                    let message = ServerMessage {
                        board: *game.board(),
                        msg: None,
                        err: Some(format!("Invalid move: {:?}", e)),
                    };

                    let payload = serde_json::to_string(&message).unwrap();
                    let _ = state.tx.send(payload);
                    continue;
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut broadcaster) => listener.abort(),
        _ = (&mut listener) => broadcaster.abort()
    }
}
