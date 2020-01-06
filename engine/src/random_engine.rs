use base::player::Player;
use base::moves::Move;
use base::state::State;
use rand::prelude::ThreadRng;
use rand::seq::IteratorRandom;


#[derive(Default)]
pub struct Engine {
  rng: ThreadRng
}

impl Engine {
  pub fn new() -> Engine {
    Engine { rng: rand::thread_rng() }
  }
}

impl Player for Engine {
  fn post_move(&self, state: &State) -> Move {
    let mut rng = self.rng;
    state.pseudo_legal_moves().choose(&mut rng).unwrap()
  }
}
