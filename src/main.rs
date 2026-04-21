mod cli;
mod game;

use std::io;

fn main() -> std::io::Result<()> {
    let reader = io::stdin().lock();
    let writer = io::stdout();

    println!("Tic Tac Toe");
    cli::run(reader, writer)?;

    Ok(())
}
