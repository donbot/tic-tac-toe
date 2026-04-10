use std::fmt::{self};

pub type Board = [Option<Player>; 9];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::O => write!(f, "O"),
            Player::X => write!(f, "X"),
        }
    }
}

pub struct GameState {
    board: Board,
}

#[derive(Debug, PartialEq)]
pub enum MarkSpaceError {
    OutOfBounds,
    SpaceTaken,
}

impl GameState {
    pub fn mark_space(&mut self, idx: usize, player: Player) -> Result<(), MarkSpaceError> {
        let space = self.board.get_mut(idx).ok_or(MarkSpaceError::OutOfBounds)?;
        if space.is_some() {
            return Err(MarkSpaceError::SpaceTaken);
        }

        *space = Some(player);
        Ok(())
    }

    pub fn check_winner(&self) -> Option<Player> {
        const WIN_CONDITIONS: [[usize; 3]; 8] = [
            [0, 1, 2], // rows
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6], // columns
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8], // diagonals
            [2, 4, 6],
        ];

        WIN_CONDITIONS.iter().find_map(|&[a, b, c]| {
            let p = self.board[a]?;
            (p == self.board[b]? && p == self.board[c]?).then_some(p)
        })
    }

    pub fn new() -> Self {
        Self { board: [None; 9] }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_space_errors() {
        let mut state = GameState::new();
        let err1 = state
            .mark_space(20, Player::X)
            .expect_err("should fail because idx is OOB");

        assert_eq!(err1, MarkSpaceError::OutOfBounds);

        state.board[2] = Some(Player::O);
        let err2 = state
            .mark_space(2, Player::X)
            .expect_err("should fail because space is taken");

        assert_eq!(err2, MarkSpaceError::SpaceTaken);
    }

    #[test]
    fn test_mark_space_updates_board() {
        let mut state = GameState::new();
        state
            .mark_space(0, Player::X)
            .expect("should mark an empty space");

        assert_eq!(state.board[0], Some(Player::X));
        assert_eq!(state.board[1], None);
    }

    #[test]
    fn test_check_winner_logic() {
        let scenarios = vec![
            ("Top Row", vec![0, 1, 2], Some(Player::X)),
            ("Middle Row", vec![3, 4, 5], Some(Player::X)),
            ("Bottom Row", vec![6, 7, 8], Some(Player::X)),
            ("Left Column", vec![0, 3, 6], Some(Player::X)),
            ("Middle Column", vec![1, 4, 7], Some(Player::X)),
            ("Right Column", vec![2, 5, 8], Some(Player::X)),
            ("Main Diagonal", vec![0, 4, 8], Some(Player::X)),
            ("Anti-Diagonal", vec![2, 4, 6], Some(Player::X)),
            ("No Winner yet", vec![0, 1], None),
        ];

        for (name, indices, expected) in scenarios {
            let mut state = GameState::new();
            for i in indices {
                state.mark_space(i, Player::X).unwrap();
            }
            assert_eq!(
                state.check_winner(),
                expected,
                "Failed on scenario: {}",
                name.to_string()
            )
        }
    }
}
