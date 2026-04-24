pub mod session;
pub mod types;

use crate::game::Game;
use crate::interface::web::types::{Lobby, Seats};
use axum::{
    Router,
    extract::{State as AxumState, ws::WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

pub async fn start(listener: tokio::net::TcpListener) -> std::io::Result<()> {
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(Lobby {
        game: Mutex::new(Game::new()),
        seats: Mutex::new(Seats::default()),
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
    ws.on_upgrade(|socket| session::run_game_session(socket, state))
}
