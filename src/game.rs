use std::thread;
use std::time::Duration;
use uci::engine::UciEngine;
use crate::cli::CLI;
use engine::engine::Engine;
use base::prelude::Color::{White,Black};
use std::collections::BTreeMap;
use base::player::Player;
use base::prelude::*;
use base::prelude::KingState::{Safe, Check};

pub struct Game {
  state: State,
  players: BTreeMap<Color, Box<dyn Player>>,
  observe: bool,
}

impl Game {
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

          if self.observe {
            println!("{}", self.state.board);
            thread::sleep(Duration::from_millis(200))
          }
          self.state.execute(mv);
        },
        _ => { panic!("Game over") }
      }
    }
  }
}

pub struct GameBuilder {
  player1: String,
  player2: String,
  observe: bool,
}

impl GameBuilder {
  pub fn new() -> GameBuilder {
    GameBuilder {
      player1: "cli".to_owned(),
      player2: "engine".to_owned(),
      observe: false
    }
  }

  pub fn player1(&mut self, player_type: String) -> &mut GameBuilder {
    self.player1 = player_type;
    self
  }

  pub fn player2(&mut self, player_type: String) -> &mut GameBuilder {
    self.player2 = player_type;
    self
  }

  pub fn observe(&mut self, observe: bool) -> &mut GameBuilder {
    self.observe = observe;
    self
  }

  pub fn build(&self) -> Game {
    let mut players = BTreeMap::new();
    players.insert(White, parse_player_type(&self.player1));
    players.insert(Black, parse_player_type(&self.player2));
    Game { state: State::new(), players, observe: self.observe }
  }
}

fn parse_player_type(string: &str) -> Box<dyn Player> {
  match string {
    "cli" => Box::from(CLI {}),
    "engine" => Box::from(Engine::new()),
    "uci" => Box::from(UciEngine {}),
    unknown => panic!(format!("Unexecpted player type {}", unknown))
  }
}
