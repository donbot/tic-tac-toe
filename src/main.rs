mod display;
mod game;

use game::Board;

fn main() {
    let board: Board = [None; 9];

    let mut buffer = Vec::new();
    display::render_board(&mut buffer, &board);
}
