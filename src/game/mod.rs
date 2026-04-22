mod state;

pub use state::Status;
pub use state::{Board, MarkSpaceError, Player, State};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Action {
    Move(usize),
    Quit,
    Invalid,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim().to_lowercase();

        if input == "quit" || input == "q" {
            return Ok(Action::Quit);
        }

        let action = input
            .parse::<usize>()
            .ok()
            .filter(|&n| n >= 1 && n <= 9)
            .map(|n| Action::Move(n - 1))
            .unwrap_or(Action::Invalid);

        Ok(action)
    }
}

pub struct Game {
    state: State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    pub fn board(&self) -> &Board {
        self.state.board()
    }

    pub fn current_player(&self) -> Player {
        self.state.current_player()
    }

    pub fn make_move(&mut self, idx: usize) -> Result<Status, MarkSpaceError> {
        let player = self.current_player();
        self.state.mark_space(idx, player)?;

        let status = self.state.status();
        if let Status::InProgress = status {
            self.state.next_turn();
        }

        Ok(status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod make_move {
        use super::*;

        #[test]
        fn does_not_switch_player_on_error() {
            let mut game = Game::new();
            let first_player = game.current_player();

            game.make_move(20).unwrap_err();
            assert_eq!(game.current_player(), first_player);
        }

        #[test]
        fn switches_player_if_game_still_in_pogress() {
            let mut game = Game::new();
            let first_player = game.current_player();

            let status = game.make_move(0).unwrap();

            assert_eq!(status, Status::InProgress, "game should be in progress");
            assert_ne!(game.current_player(), first_player);
        }

        #[test]
        fn does_not_switch_player_when_game_ends() {
            let mut game = Game::new();
            let first_player = game.current_player();

            game.state.mark_space(0, first_player).unwrap();
            game.state.mark_space(1, first_player).unwrap();

            let status = game.make_move(2).unwrap();

            assert_eq!(status, Status::Won(first_player), "game should end");
            assert_eq!(game.current_player(), first_player);
        }
    }
}
