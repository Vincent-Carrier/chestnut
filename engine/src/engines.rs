use core::moves::Move;
use core::state::State;

pub trait Engine {
  fn best_move(&self, state: &State) -> Move;
}
