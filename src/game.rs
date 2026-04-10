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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_space_errors() {
        let mut state = GameState { board: [None; 9] };
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
        let mut state = GameState { board: [None; 9] };
        state
            .mark_space(0, Player::X)
            .expect("should mark an empty space");

        assert_eq!(state.board[0], Some(Player::X));
        assert_eq!(state.board[1], None);
    }
}
