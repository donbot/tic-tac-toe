use crate::game::Board;
use std::io::{Error, Write};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Player;

    #[test]
    fn test_print_board_empty() {
        let board = [None; 9];
        let mut buffer = Vec::new();

        render_board(&mut buffer, &board).expect("Should render a board with just numbers");

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains(" 1 | 2 | 3 "));
        assert!(output.contains(" 4 | 5 | 6 "));
        assert!(output.contains(" 7 | 8 | 9 "));
    }

    #[test]
    fn test_print_board_with_players() {
        let mut board = [None; 9];
        board[0] = Some(Player::X);
        board[4] = Some(Player::O);

        let mut buffer = Vec::new();
        render_board(&mut buffer, &board).expect("Should render an X and an O mark");

        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains(" X | 2 | 3 "));
        assert!(output.contains(" 4 | O | 6 "));
    }
}
