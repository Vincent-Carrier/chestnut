use base::prelude::Color::{White,Black};
use std::collections::BTreeMap;
use base::player::Player;
use base::prelude::*;
use base::prelude::KingState::{Safe, Check};

pub struct Game {
  state: State,
  players: BTreeMap<Color, Box<dyn Player>>
}

impl Game {
  pub fn new(player1: Box<dyn Player>, player2: Box<dyn Player>) -> Game {
    let mut players = BTreeMap::new();
    players.insert(White, player1);
    players.insert(Black, player2);
    Game { state: State::new(), players }
  }

  pub fn start(&mut self) {
    loop {
      match self.state.king_state {
        Safe | Check => {
          let active = &self.state.active_color;
          let player = &self.players[active];
          let other_player = &self.players[&active.opposite()];
          player.prompt_turn(&self.state);
          let mv = player.post_move(&self.state);
          other_player.accept_move(&mv, &self.state);
          self.state.execute(mv);
        },
        _ => { panic!("Game over") }
      }
    }
  }
}
