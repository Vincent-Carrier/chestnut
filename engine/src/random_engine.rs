use crate::engines::Engine;
use base::moves::Move;
use base::state::State;
use rand::prelude::ThreadRng;
use rand::seq::IteratorRandom;


#[derive(Default)]
pub struct RandomEngine {
  rng: ThreadRng
}

impl RandomEngine {
  pub fn new() -> RandomEngine {
    RandomEngine { rng: rand::thread_rng() }
  }
}

impl Engine for RandomEngine {
  fn best_move(&mut self, state: &State) -> Move {
    state.pseudo_legal_moves().choose(&mut self.rng).unwrap()
  }
}

impl Player for Engine {

}
