mod display;
mod game;
mod input;

use crate::game::GameState;

fn main() {
    let state = GameState::new();

    display::render_board(&mut std::io::stdout(), state.board()).expect("Failed to render board");
}
