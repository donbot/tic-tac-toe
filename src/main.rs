mod display;
mod game;
mod input;

use crate::game::{GameState, GameStatus};
use std::io;

fn main() -> std::io::Result<()> {
    let mut state = GameState::new();
    let mut stdout = io::stdout();
    let mut stdin = io::stdin().lock();

    println!("Tic Tac Toe");
    loop {
        display::render_board(&mut stdout, state.board())?;

        println!("Player {}, enter your move (1-9)", state.current_player());
        let move_idx = match input::get_move(&mut stdin) {
            Some(idx) => idx,
            None => {
                println!("Invalid input. Please enter a number between 1 and 9.");
                continue;
            }
        };

        if let Err(e) = state.make_move(move_idx) {
            println!("Error: {:?}. try again.", e);
            continue;
        }

        match state.status() {
            GameStatus::Won(winner) => {
                display::render_board(&mut stdout, state.board())?;
                println!("Player {} wins!", winner);
                break;
            }
            GameStatus::Draw => {
                display::render_board(&mut stdout, state.board())?;
                println!("It's a Draw!");
                break;
            }
            GameStatus::InProgress => continue,
        }
    }
    Ok(())
}
