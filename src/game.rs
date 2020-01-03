use crate::player::Player::Computer;
use crate::player::Player::Human;
use std::collections::HashMap;
use crate::player::Player;
use crate::ui::CLI;
use engine::random_engine::RandomEngine;
use base::state::KingState::Checkmate;
use base::prelude::*;
use base::prelude::Color::{White, Black};

pub struct Game {
  state: State,
  players: HashMap<Color, Player>
}

impl Game {
  pub fn new() -> Game {
    let mut players = HashMap::new();
    let engine = RandomEngine::new();
    players.insert(White, Human { ui: CLI::new() });
    players.insert(Black, Computer { engine: Box::from(engine) });
    Game { state: State::new(), players }
  }

  pub fn start(&mut self) {
    while self.state.king_state != Checkmate {
      let player = self.players[&self.state.active_color];
      let mv = match player {
        Human { ui } => {
          ui.prompt_turn(&self.state);
          loop {
            if let Some(m) = ui.prompt_move(&self.state) { break m } else { continue }
          }
        },
        Computer { engine } => {
          engine.best_move(&self.state)
        }
      };
      self.state.execute(mv);
    }
  }
}
