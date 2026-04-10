use crate::game::Board;
use std::io::Write;

pub fn render_board<W: Write>(writer: &mut W, board: &Board) {
    let display_board: [String; 9] = std::array::from_fn(|i| match board[i] {
        Some(player) => player.to_string(),
        None => (i + 1).to_string(),
    });

    println!("-----------");
    for row in display_board.chunks(3) {
        writeln!(writer, " {} | {} | {} ", row[0], row[1], row[2]).unwrap();
        println!("-----------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Player;

    #[test]
    fn test_print_board_empty() {
        let board = [None; 9];
        let mut buffer = Vec::new();

        render_board(&mut buffer, &board);

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
        render_board(&mut buffer, &board);

        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains(" X | 2 | 3 "));
        assert!(output.contains(" 4 | O | 6 "));
    }
}
