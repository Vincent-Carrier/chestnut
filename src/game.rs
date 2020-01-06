use crate::uci_engine::UciEngine;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::stdin;
use base::prelude::Color::{White,Black};
use std::collections::BTreeMap;
use base::player::Player;
use engine::random_engine::RandomEngine;
use base::prelude::*;
use base::prelude::KingState::{Checkmate};
use vampirc_uci::{UciMessage, UciFen, Serializable};

pub struct Game {
  state: State,
  players: BTreeMap<Color, Box<dyn Player>>
}

impl Game {
  pub fn new() -> Game {
    let mut players = BTreeMap::new();
    let engine = RandomEngine::new();
    let uci = UciEngine { reader: BufReader::new(stdin()) };
    players.insert(White, Box::from(engine));
    players.insert(Black, Box::from(uci));
    Game { state: State::new(), players }
  }

  pub fn start(&mut self) {
    while self.state.king_state != Checkmate {
      let player = self.players[&self.state.active_color];
      let playing_against = self.players[&self.state.active_color.opposite()];
      let mv = player.post_move(&self.state);
      playing_against.accept_move(mv, &self.state);
      self.state.execute(mv);
    }
  }
}
