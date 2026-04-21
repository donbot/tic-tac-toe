mod state;

pub use state::Status;
pub use state::{Board, MarkSpaceError, Player, State};

#[derive(Debug, PartialEq)]
pub enum Action {
    Move(usize),
    Quit,
    Invalid,
}

impl Action {
    pub fn from_str(input: &str) -> Self {
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
}

pub struct Game {
    pub state: State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }
}
