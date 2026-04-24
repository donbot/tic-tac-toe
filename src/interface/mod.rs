pub mod cli;
pub mod web;

use crate::game::{ActionError, Board, Event, Game, Player, Status};
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum GameMessage {
    Update {
        board: Board,
        message: String,
    },
    GameOver {
        board: Board,
        message: String,
        winner: Option<Player>,
    },
    Error {
        message: String,
    },
    Quit {
        message: String,
    },
}

impl GameMessage {
    pub fn from_event(game: &Game, event: &Event) -> Self {
        match event {
            Event::BoardUpdate => Self::Update {
                board: *game.board(),
                message: format!("Player {}'s turn", game.current_player()),
            },
            Event::MoveError(err) => Self::Error {
                message: format!("Illegal move: {:?}", err),
            },
            Event::Quit => Self::Quit {
                message: "Game exited.".into(),
            },
            Event::GameOver(status) => {
                let (winner, msg) = match status {
                    Status::Won(p) => (Some(*p), format!("Player {} wins!", p)),
                    Status::Draw => (None, "It's a draw!".into()),
                    _ => (None, String::new()),
                };

                Self::GameOver {
                    board: *game.board(),
                    message: msg,
                    winner,
                }
            }
        }
    }

    pub fn from_error(err: &ActionError) -> Self {
        let err_msg = match err {
            ActionError::NotANumber => "Please enter a valid digit (1-9).",
            ActionError::OutOfRange => "Number is out of bounds! Use 1-9.",
        };

        Self::Error {
            message: err_msg.into(),
        }
    }
}
