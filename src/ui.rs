use crate::game::{Board, MarkSpaceError, Player};
use std::io::{self, BufRead, Error, Write};

#[derive(Debug, PartialEq)]
pub enum Action {
    Move(usize),
    Quit,
    Invalid,
}

pub fn get_action<R: BufRead>(reader: &mut R) -> Action {
    let mut input = String::new();

    if reader.read_line(&mut input).unwrap_or(0) == 0 {
        return Action::Quit;
    }

    let input = input.trim().to_lowercase();

    if input == "quit" || input == "q" {
        return Action::Quit;
    }

    input
        .parse::<usize>()
        .ok()
        .filter(|&n| n >= 1 && n <= 9)
        .map(|n| Action::Move(n - 1))
        .unwrap_or(Action::Invalid)
}

pub fn render_board<W: Write>(writer: &mut W, board: &Board) -> Result<(), Error> {
    let get_mark = |i: usize| -> String {
        match board[i] {
            Some(player) => player.to_string(),
            None => (i + 1).to_string(),
        }
    };

    writeln!(writer)?;
    for i in (0..9).step_by(3) {
        writeln!(
            writer,
            " {} | {} | {} ",
            get_mark(i),
            get_mark(i + 1),
            get_mark(i + 2)
        )?;
        if i < 6 {
            writeln!(writer, "-----------")?;
        }
    }
    writeln!(writer)?;
    Ok(())
}

pub fn prompt_move<W: Write>(writer: &mut W, player: Player) -> io::Result<()> {
    print(
        writer,
        &format!("Player {}, enter your move (1-9):", player),
    )
}

pub fn report_invalid_input<W: Write>(writer: &mut W) -> io::Result<()> {
    print(
        writer,
        "Invalid input. Please enter a number between 1 and 9.",
    )
}

pub fn report_error<W: Write>(writer: &mut W, error: MarkSpaceError) -> io::Result<()> {
    print(writer, &format!("Error: {:?}. Try again.", error))
}

pub fn announce_winner<W: Write>(writer: &mut W, winner: Player) -> io::Result<()> {
    print(writer, &format!("Player {} wins!", winner))
}

pub fn announce_draw<W: Write>(writer: &mut W) -> io::Result<()> {
    print(writer, "It's a Draw!")
}

pub fn announce_quit<W: Write>(writer: &mut W) -> io::Result<()> {
    print(writer, "Game exited.")
}

fn print<W: Write>(writer: &mut W, msg: &str) -> io::Result<()> {
    writeln!(writer, "{}", msg)?;
    writer.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod render_board {
        use super::*;

        #[test]
        fn renders_an_empty_board() {
            let board = [None; 9];
            let mut buffer = Vec::new();

            render_board(&mut buffer, &board).expect("Should render a board with just numbers");

            let output = String::from_utf8(buffer).unwrap();
            assert!(output.contains(" 1 | 2 | 3 "));
            assert!(output.contains(" 4 | 5 | 6 "));
            assert!(output.contains(" 7 | 8 | 9 "));
        }

        #[test]
        fn renders_board_with_both_players() {
            let mut board = [None; 9];
            board[0] = Some(Player::X);
            board[4] = Some(Player::O);

            let mut buffer = Vec::new();
            render_board(&mut buffer, &board).expect("Should render an X and an O mark");

            let output = String::from_utf8(buffer).unwrap();

            assert!(output.contains(" X | 2 | 3 "));
            assert!(output.contains(" 4 | O | 6 "));
            assert!(output.contains(" 7 | 8 | 9 "));
        }
    }

    mod get_action {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn returns_move_action_if_input_passes_validation() {
            let mut input = Cursor::new("5\n");
            let result = get_action(&mut input);

            assert_eq!(result, Action::Move(4));
        }

        #[test]
        fn returns_invalid_action_if_oob_or_input_is_invalid() {
            let scenarios = [("Out of Bounds", "9000\n"), ("Invalid Input", "potato\n")];

            for (name, input_text) in scenarios {
                let mut input = Cursor::new(input_text);
                let result = get_action(&mut input);
                assert_eq!(result, Action::Invalid, "scenario \"{}\"", name);
            }
        }

        #[test]
        fn returns_quit_action_on_quit_command_or_eof() {
            let scenarios = [
                ("Quit", "quit\n"),
                ("Quit Shortcut", "q\n"),
                ("End of File", ""),
            ];

            for (name, input_text) in scenarios {
                let mut input = Cursor::new(input_text);
                let result = get_action(&mut input);
                assert_eq!(result, Action::Quit, "scenario \"{}\"", name);
            }
        }
    }
}
