use crate::board::*;
use crate::moves::*;
use crate::player::*;

pub type History = Vec<Box<dyn Move>>;

pub struct State {
  pub board: Board,
  pub history: History,
  pub active_player: Player,
  pub passive_player: Player,
  pub in_check: bool,
}
