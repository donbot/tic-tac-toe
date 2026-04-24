use futures_util::{SinkExt, StreamExt};
use tic_tac_toe::{
    game::Player,
    interface::{
        GameMessage,
        web::{self, types::WebAction},
    },
};
use tokio::net::TcpListener;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::test]
async fn test_game_flow() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async {
        web::start(listener).await.expect("Server failed to start");
    });

    let url = format!("ws://{}/ws", addr);

    // connect Player X
    let (ws_x, _) = connect_async(&url).await.unwrap();
    let (mut tx_x, mut rx_x) = ws_x.split();
    assert!(matches!(recv(&mut rx_x).await, GameMessage::Update { .. }));

    // connect Player O
    let (ws_o, _) = connect_async(&url).await.unwrap();
    let (mut tx_o, mut rx_o) = ws_o.split();
    assert!(matches!(recv(&mut rx_o).await, GameMessage::Update { .. }));

    // Player X claims seat X
    send(&mut tx_x, WebAction::Claim(Player::X)).await;
    let claim_x = recv(&mut rx_x).await;
    let _ = recv(&mut rx_o).await;
    assert!(
        matches!(claim_x, GameMessage::Update { ref message, .. } if message.contains("Player X joined"))
    );

    // Player O claims seat O
    send(&mut tx_o, WebAction::Claim(Player::O)).await;
    let claim_o = recv(&mut rx_o).await;
    let _ = recv(&mut rx_x).await;
    assert!(
        matches!(claim_o, GameMessage::Update { ref message, .. } if message.contains("Player O joined"))
    );

    // X makes valid move.
    send(&mut tx_x, WebAction::Move(1)).await;
    let move_x = recv(&mut rx_x).await;
    let _ = recv(&mut rx_o).await;
    if let GameMessage::Update { board, .. } = move_x {
        assert_eq!(board[0], Some(Player::X));
    }

    // X's out of turn move is ignored.
    send(&mut tx_x, WebAction::Move(2)).await;

    // O makes valid move.
    send(&mut tx_o, WebAction::Move(4)).await;
    let move_o = recv(&mut rx_o).await;
    let _ = recv(&mut rx_x).await;
    if let GameMessage::Update { board, .. } = move_o {
        assert_eq!(board[3], Some(Player::O));
    }

    // X quits. O receives broadcast of X quitting.
    send(&mut tx_x, WebAction::Quit).await;
    assert!(
        matches!(recv(&mut rx_o).await, GameMessage::Update { ref message, .. } if message.contains("left"))
    );
}

async fn send<S>(tx: &mut S, action: WebAction)
where
    S: SinkExt<Message> + Unpin,
    <S as futures_util::Sink<Message>>::Error: std::fmt::Debug,
{
    let json = serde_json::to_string(&action).expect("Failed to serialize WebAction");
    tx.send(Message::Text(json.into()))
        .await
        .expect("Failed to send message");
}

async fn recv<S>(rx: &mut S) -> GameMessage
where
    S: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let msg = rx
        .next()
        .await
        .unwrap()
        .expect("Stream closed unexpectedly");
    serde_json::from_str(msg.to_text().unwrap()).expect("Failed to parse GameMessage JSON")
}
