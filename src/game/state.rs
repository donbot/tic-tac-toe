use serde::{Deserialize, Serialize};
use std::fmt::{self};

pub type Board = [Option<Player>; 9];

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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

pub struct State {
    board: Board,
    current_player: Player,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: [None; 9],
            current_player: Player::X,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_player(&self) -> Player {
        self.current_player
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

    pub(super) fn mark_space(&mut self, idx: usize, player: Player) -> Result<(), MarkSpaceError> {
        let space = self.board.get_mut(idx).ok_or(MarkSpaceError::OutOfBounds)?;
        if space.is_some() {
            return Err(MarkSpaceError::SpaceTaken);
        }

        *space = Some(player);
        Ok(())
    }

    pub(super) fn next_turn(&mut self) {
        self.current_player = self.current_player.next();
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

    mod mark_space {
        use super::*;

        #[test]
        fn fails_when_index_is_out_of_bounds() {
            let mut state = State::new();
            let err = state
                .mark_space(20, state.current_player())
                .expect_err("should fail because idx is OOB");

            assert_eq!(err, MarkSpaceError::OutOfBounds);
        }
        #[test]
        fn fails_when_space_is_taken() {
            let mut state = State::new();
            state.mark_space(2, state.current_player()).unwrap();
            let err = state
                .mark_space(2, state.current_player())
                .expect_err("should fail because space is taken");

            assert_eq!(err, MarkSpaceError::SpaceTaken);
        }

        #[test]
        fn assigns_index_to_current_player() {
            let mut state = State::new();
            state
                .mark_space(0, state.current_player())
                .expect("should mark an empty space");

            assert_eq!(
                state.board[0],
                Some(state.current_player()),
                "should be occupied by current player"
            );
            assert_eq!(state.board[1], None, "should still be empty");
        }
    }

    #[test]
    fn check_winner_scenarios() {
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
                "scenario: {}",
                name.to_string()
            )
        }
    }

    #[test]
    fn status_scenarios() {
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
                let player = state.current_player();
                state.mark_space(idx, player).unwrap();
                state.next_turn();
            }

            assert_eq!(state.status(), expected, "scenario: {}", name)
        }
    }
}
