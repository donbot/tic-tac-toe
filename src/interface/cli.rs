use crate::{
    game::{Action, ActionError, Board, Game, Player},
    interface::GameMessage,
};
use std::io::{self, BufRead, Error, Write};

pub fn run<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> std::io::Result<()> {
    let mut game = Game::new();

    loop {
        render_board(&mut writer, game.board())?;
        prompt_move(&mut writer, game.current_player())?;

        let action = match get_action(&mut reader) {
            Ok(a) => a,
            Err(e) => {
                let err_msg = GameMessage::from_error(&e);
                if let GameMessage::Error { message } = err_msg {
                    writeln!(writer, "{}", message)?;
                }
                continue;
            }
        };

        let event = game.process_action(action);
        let msg = GameMessage::from_event(&game, &event);

        match msg {
            GameMessage::Update { message, .. } => {
                writeln!(writer, "{}", message)?;
            }
            GameMessage::Error { message } => {
                writeln!(writer, "{}", message)?;
                continue;
            }
            GameMessage::Quit { message } => {
                writeln!(writer, "{}", message)?;
                break;
            }
            GameMessage::GameOver { board, message, .. } => {
                render_board(&mut writer, &board)?;
                writeln!(writer, "{}", message)?;
                break;
            }
        }
    }
    Ok(())
}

pub fn get_action<R: BufRead>(reader: &mut R) -> Result<Action, ActionError> {
    let mut input = String::new();

    if reader.read_line(&mut input).unwrap_or(0) == 0 {
        return Ok(Action::Quit);
    }
    input.trim().parse::<Action>()
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
            let result = get_action(&mut input).unwrap();

            assert_eq!(result, Action::Move(4));
        }

        #[test]
        fn returns_action_error_if_oob_or_input_is_invalid() {
            let scenarios = [
                ("Out of Bounds", "9000\n", ActionError::OutOfRange),
                ("Invalid Input", "potato\n", ActionError::NotANumber),
            ];

            for (name, input_text, expected) in scenarios {
                let mut input = Cursor::new(input_text);
                let result = get_action(&mut input).unwrap_err();
                assert_eq!(result, expected, "scenario \"{}\"", name);
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
                let result = get_action(&mut input).unwrap();
                assert_eq!(result, Action::Quit, "scenario \"{}\"", name);
            }
        }
    }

    mod run {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn runs_until_game_is_complete() {
            // X | X | X
            // O | O |
            //   |   |
            let input = b"1\n4\n2\n5\n3\n";
            let mut output = Vec::new();

            let _ = run(Cursor::new(input), &mut output);

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

            let _ = run(Cursor::new(input), &mut output);

            let result = String::from_utf8(output).unwrap();

            assert!(
                result.contains("Please enter a valid digit"),
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
            let _ = run(Cursor::new(input), &mut output);

            let result = String::from_utf8(output).unwrap();
            assert!(result.contains("Game exited."));
        }
    }
}
