use base::prelude::*;

// We need to make this a trait because we dont't
// want UCI engines for WebAssembly target
pub trait Player {
  fn prompt_turn(state: &State);
  fn accept_move(mv: &Move);
  fn post_move(state: &State) -> Move;
}

