use crate::prelude::*;

// We need to make this a trait because we dont't
// want UCI engines for WebAssembly target
pub trait Player: Default {
  fn game_start(&self, state: &State);
  fn prompt_turn(&self, state: &State);
  fn accept_move(&self, mv: &Move);
  fn post_move(&self, state: &State) -> Move;
}

