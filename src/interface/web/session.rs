use crate::{
    game::Player,
    interface::GameMessage,
    interface::web::types::{Lobby, NEXT_USER_ID, WebAction},
};
use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub async fn run_game_session(socket: WebSocket, state: Arc<Lobby>) {
    let user_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
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

    let state_clone = state.clone();
    let mut listener = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let action: WebAction = match serde_json::from_str(&text) {
                Ok(a) => a,
                Err(_) => continue,
            };

            if let WebAction::Quit = action {
                break;
            }

            let broadcast_msg = {
                let mut game = state_clone.game.lock().unwrap();
                let mut seats = state_clone.seats.lock().unwrap();
                action.execute(user_id, &mut game, &mut seats)
            };

            if let Some(msg) = broadcast_msg {
                let payload = serde_json::to_string(&msg).unwrap();
                let _ = state_clone.tx.send(payload);
            }
        }
        let mut seats = state.seats.lock().unwrap();
        let mut vacated = None;

        if seats.x_owner == Some(user_id) {
            seats.x_owner = None;
            vacated = Some(Player::X);
        } else if seats.o_owner == Some(user_id) {
            seats.o_owner = None;
            vacated = Some(Player::O);
        }

        if let Some(p) = vacated {
            let msg = GameMessage::Update {
                board: *state.game.lock().unwrap().board(),
                message: format!("Player {} left.", p),
            };
            let _ = state.tx.send(serde_json::to_string(&msg).unwrap());
        }
    });

    tokio::select! {
        _ = (&mut broadcaster) => listener.abort(),
        _ = (&mut listener) => broadcaster.abort()
    }
}
