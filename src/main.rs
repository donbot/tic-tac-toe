mod cli;
mod game;
mod web;

use std::io;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--web".to_string()) {
        web::start().await;
    } else {
        let reader = io::stdin().lock();
        let writer = io::stdout();

        println!("Tic Tac Toe");
        cli::run(reader, writer)?;
    }

    Ok(())
}
