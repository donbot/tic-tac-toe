mod state;

use crate::ui;
pub use state::{Board, MarkSpaceError, Player};
use state::{State, Status};
use std::io::{BufRead, Write};

pub struct Game<R: BufRead, W: Write> {
    state: State,
    reader: R,
    writer: W,
}

impl<R: BufRead, W: Write> Game<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            state: State::new(),
            reader,
            writer,
        }
    }
    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            ui::render_board(&mut self.writer, self.state.board())?;

            ui::prompt_move(&mut self.writer, self.state.current_player())?;
            let move_idx = match ui::get_action(&mut self.reader) {
                ui::Action::Move(idx) => idx,
                ui::Action::Invalid => {
                    ui::report_invalid_input(&mut self.writer)?;
                    continue;
                }
                ui::Action::Quit => {
                    ui::announce_quit(&mut self.writer)?;
                    break;
                }
            };

            if let Err(e) = self.state.make_move(move_idx) {
                ui::report_error(&mut self.writer, e)?;
                continue;
            }

            match self.state.status() {
                Status::Won(winner) => {
                    ui::render_board(&mut self.writer, self.state.board())?;
                    ui::announce_winner(&mut self.writer, winner)?;
                    break;
                }
                Status::Draw => {
                    ui::render_board(&mut self.writer, self.state.board())?;
                    ui::announce_draw(&mut self.writer)?;
                    break;
                }
                Status::InProgress => continue,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn runs_until_game_is_complete() {
        // X | X | X
        // O | O |
        //   |   |
        let input = b"1\n4\n2\n5\n3\n";
        let mut output = Vec::new();

        let mut game = Game::new(Cursor::new(input), &mut output);
        game.run().unwrap();

        let result = String::from_utf8(output).unwrap();

        assert!(
            result.contains("Player X wins!"),
            "should announce X as the winner"
        );
    }

    #[test]
    fn retries_until_input_is_valid() {
        let input = b"abc\n1\n1\n4\n2\n5\n3\n";
        let mut output = Vec::new();

        let mut game = Game::new(Cursor::new(input), &mut output);
        game.run().unwrap();

        let result = String::from_utf8(output).unwrap();

        assert!(
            result.contains("Invalid input"),
            "should have warned about invalid input"
        );
        assert!(
            result.contains("Player X wins!"),
            "should continue to completion after errors"
        );
    }
    #[test]
    fn quits_on_command() {
        let input = b"quit\n";
        let mut output = Vec::new();
        let mut game = Game::new(Cursor::new(input), &mut output);
        game.run().unwrap();

        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("Game exited."));
    }
}
