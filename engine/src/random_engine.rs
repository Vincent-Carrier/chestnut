use rand::prelude::ThreadRng;
use base::state::State;
use base::moves::Move;
use crate::engine::Engine;
use rand::seq::IteratorRandom;


pub struct RandomEngine {
  rng: ThreadRng
}

impl RandomEngine {
  pub fn new() -> RandomEngine {
    RandomEngine { rng: rand::thread_rng() }
  }
}

impl Engine for RandomEngine {
  fn best_move(&self, state: &State) -> Move {
    state.pseudo_legal_moves().choose(&mut self.rng).unwrap()
  }
}
