use futures_util::{SinkExt, StreamExt};
use tic_tac_toe::{
    game::Player,
    interface::{GameMessage, web},
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
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut tx, mut rx) = ws_stream.split();

    async fn recv<S>(rx: &mut S) -> GameMessage
    where
        S: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        let msg = rx.next().await.unwrap().unwrap();
        serde_json::from_str(msg.to_text().unwrap()).unwrap()
    }

    let msg = rx.next().await.unwrap().unwrap();
    assert!(matches!(msg, Message::Text(ref t) if t.contains("Game Started!")));

    tx.send(Message::Text("1".into())).await.unwrap();
    assert!(
        matches!(recv(&mut rx).await, GameMessage::Update { board, .. } if board[0] == Some(Player::X))
    );

    tx.send(Message::Text("1".into())).await.unwrap();
    assert!(
        matches!(recv(&mut rx).await, GameMessage::Error { message } if message.contains("SpaceTaken"))
    );

    tx.send(Message::Text("99".into())).await.unwrap();
    assert!(matches!(recv(&mut rx).await, GameMessage::Error { .. }));

    tx.send(Message::Text("quit".into())).await.unwrap();
    assert!(matches!(recv(&mut rx).await, GameMessage::Quit { .. }));
}
