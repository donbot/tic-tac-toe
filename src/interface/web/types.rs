use crate::game::{Action, Game, Player};
use crate::interface::GameMessage;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, atomic::AtomicUsize};
use tokio::sync::broadcast;

pub(super) static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WebAction {
    Claim(Player),
    Move(usize),
    Quit,
}

#[derive(Default)]
pub struct Seats {
    pub x_owner: Option<usize>,
    pub o_owner: Option<usize>,
}

pub struct Lobby {
    pub game: Mutex<Game>,
    pub tx: broadcast::Sender<String>,
    pub seats: Mutex<Seats>,
}

impl WebAction {
    pub fn execute(
        self,
        player_id: usize,
        game: &mut Game,
        seats: &mut Seats,
    ) -> Option<GameMessage> {
        match self {
            WebAction::Claim(player) => {
                let (owner, other_owner) = match player {
                    Player::X => (&mut seats.x_owner, seats.o_owner),
                    Player::O => (&mut seats.o_owner, seats.x_owner),
                };

                if owner.is_none() && other_owner != Some(player_id) {
                    *owner = Some(player_id);
                    return Some(GameMessage::Update {
                        board: *game.board(),
                        message: format!("Player {} joined!", player),
                    });
                }
                None
            }
            WebAction::Move(idx) => {
                let current_owner = match game.current_player() {
                    Player::X => seats.x_owner,
                    Player::O => seats.o_owner,
                };

                if current_owner == Some(player_id) {
                    let event = game.process_action(Action::Move(idx - 1));
                    return Some(GameMessage::from_event(game, &event));
                }
                None
            }
            WebAction::Quit => None,
        }
    }
}
