use uci::engine::UciEngine;
use base::prelude::Color::{White,Black};
use std::collections::BTreeMap;
use base::player::Player;
use crate::cli::CLI;
use base::prelude::*;
use base::prelude::KingState::{Safe, Check};

pub struct Game {
  state: State,
  players: BTreeMap<Color, Box<dyn Player>>
}

impl Game {
  pub fn new() -> Game {
    let mut players = BTreeMap::new();
    let player1: Box<dyn Player> = Box::from(CLI {});
    let player2: Box<dyn Player> = Box::from(UciEngine {});
    players.insert(White, player1);
    players.insert(Black, player2);
    Game { state: State::new(), players }
  }

  pub fn start(&mut self) {
    loop {
      match self.state.king_state {
        Safe | Check => {
          let player = &self.players[&self.state.active_color];
          let playing_against = &self.players[&self.state.active_color.opposite()];
          let mv = player.post_move(&self.state);
          playing_against.accept_move(&mv, &self.state);
          self.state.execute(mv);
        },
        _ => { panic!("Game over") }
      }
    }
  }
}
