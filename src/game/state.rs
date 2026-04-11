use std::fmt::{self};

pub type Board = [Option<Player>; 9];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn next(&self) -> Self {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::O => write!(f, "O"),
            Player::X => write!(f, "X"),
        }
    }
}

pub struct State {
    board: Board,
    current_player: Player,
}

#[derive(Debug, PartialEq)]
pub enum MarkSpaceError {
    OutOfBounds,
    SpaceTaken,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    InProgress,
    Won(Player),
    Draw,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: [None; 9],
            current_player: Player::X,
        }
    }

    pub fn make_move(&mut self, idx: usize) -> Result<(), MarkSpaceError> {
        self.mark_space(idx)?;
        self.current_player = self.current_player.next();
        Ok(())
    }

    pub fn status(&self) -> Status {
        if let Some(winner) = self.check_winner() {
            return Status::Won(winner);
        }
        if self.board.iter().all(|space| space.is_some()) {
            return Status::Draw;
        }
        Status::InProgress
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    fn mark_space(&mut self, idx: usize) -> Result<(), MarkSpaceError> {
        let space = self.board.get_mut(idx).ok_or(MarkSpaceError::OutOfBounds)?;
        if space.is_some() {
            return Err(MarkSpaceError::SpaceTaken);
        }

        *space = Some(self.current_player);
        Ok(())
    }

    fn check_winner(&self) -> Option<Player> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_space_errors() {
        let mut state = State::new();
        let err1 = state
            .mark_space(20)
            .expect_err("should fail because idx is OOB");

        assert_eq!(err1, MarkSpaceError::OutOfBounds);

        state.mark_space(2).unwrap();
        let err2 = state
            .mark_space(2)
            .expect_err("should fail because space is taken");

        assert_eq!(err2, MarkSpaceError::SpaceTaken);
    }

    #[test]
    fn test_mark_space_updates_board() {
        let mut state = State::new();
        state.mark_space(0).expect("should mark an empty space");

        assert_eq!(state.board[0], Some(state.current_player()));
        assert_eq!(state.board[1], None);
    }

    #[test]
    fn test_check_winner_scenarios() {
        let scenarios = vec![
            ("Top Row", vec![0, 1, 2], Player::X, Some(Player::X)),
            ("Middle Row", vec![3, 4, 5], Player::O, Some(Player::O)),
            ("Bottom Row", vec![6, 7, 8], Player::X, Some(Player::X)),
            ("Left Column", vec![0, 3, 6], Player::O, Some(Player::O)),
            ("Middle Column", vec![1, 4, 7], Player::X, Some(Player::X)),
            ("Right Column", vec![2, 5, 8], Player::O, Some(Player::O)),
            ("Main Diagonal", vec![0, 4, 8], Player::X, Some(Player::X)),
            ("Anti-Diagonal", vec![2, 4, 6], Player::O, Some(Player::O)),
            ("No Winner yet", vec![0, 1], Player::X, None),
        ];

        for (name, indices, player, expected) in scenarios {
            let mut state = State::new();
            for i in indices {
                state.board[i] = Some(player);
            }
            assert_eq!(
                state.check_winner(),
                expected,
                "Failed on scenario: {}",
                name.to_string()
            )
        }
    }
    #[test]
    fn test_status_scenarios() {
        let scenarios = vec![
            (
                "X Wins",
                // X | X | X
                // O | O |
                //   |   |
                vec![0, 3, 1, 4, 2],
                Status::Won(Player::X),
            ),
            (
                "O Wins",
                // X | X |
                // O | O | O
                // X |   |
                vec![0, 3, 1, 4, 6, 5],
                Status::Won(Player::O),
            ),
            (
                "Draw",
                // X | O | X
                // X | O | X
                // O | X | O
                vec![0, 1, 2, 4, 3, 6, 5, 8, 7],
                Status::Draw,
            ),
            ("In Progress", vec![0], Status::InProgress),
        ];

        for (name, moves, expected) in scenarios {
            let mut state = State::new();
            for idx in moves {
                state.make_move(idx).unwrap();
            }

            assert_eq!(state.status(), expected, "Failed on scenario: {}", name)
        }
    }
    #[test]
    fn test_make_move_updates_current_player() {
        let mut state = State::new();
        assert_eq!(state.current_player(), Player::X);

        state.make_move(0).unwrap();
        assert_eq!(state.current_player(), Player::O);
    }
}
