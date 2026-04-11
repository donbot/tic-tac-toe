mod game;
mod ui;

use crate::game::Game;
use std::io;

fn main() -> std::io::Result<()> {
    let reader = io::stdin().lock();
    let writer = io::stdout();

    let mut game = Game::new(reader, writer);

    println!("Tic Tac Toe");
    game.run()
}
