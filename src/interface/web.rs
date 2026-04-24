use crate::{
    game::{Action, Game},
    interface::GameMessage,
};
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

pub async fn start(listener: tokio::net::TcpListener) -> std::io::Result<()> {
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(Lobby {
        game: Mutex::new(Game::new()),
        tx,
    });

    let app = Router::new()
        .route("/ws", any(join_game))
        .with_state(app_state);

    let local_addr = listener.local_addr()?;
    println!("Web server live at ws://{}", local_addr);
    axum::serve(listener, app).await
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
        let payload = GameMessage::Update {
            board: *game.board(),
            message: "Game Started!".into(),
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
            let mut game = state.game.lock().unwrap();
            let message = match text.trim().parse::<Action>() {
                Ok(action) => {
                    let event = game.process_action(action);
                    GameMessage::from_event(&game, &event)
                }
                Err(e) => GameMessage::from_error(&e),
            };

            let payload = serde_json::to_string(&message).unwrap();
            let _ = state.tx.send(payload);
        }
    });

    tokio::select! {
        _ = (&mut broadcaster) => listener.abort(),
        _ = (&mut listener) => broadcaster.abort()
    }
}
