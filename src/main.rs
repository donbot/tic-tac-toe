use std::io;
use tic_tac_toe::interface::{cli, web};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--web".to_string()) {
        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        web::start(listener).await?;
    } else {
        let reader = io::stdin().lock();
        let writer = io::stdout();

        println!("Tic Tac Toe");
        cli::run(reader, writer)?;
    }

    Ok(())
}
