use core::state::State;

pub trait Engine {
  fn best_move(&state: State) -> Move;
}
