use crate::player::Player::Computer;
use crate::player::Player::Human;
use std::collections::HashMap;
use crate::player::Player;
use crate::ui::CLI;
use engine::random_engine::RandomEngine;
use base::state::KingState::Checkmate;
use base::prelude::*;
use base::prelude::Color::{White, Black};
use vampirc_uci::{Serializable, UciMessage};

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
      let playing_against = self.players[&self.state.active_color.opposite()];
      let uci_mv = match player {
        Human { ui } => {
          ui.prompt_turn(&self.state);
          loop {
            if let Ok(uci_mv) = ui.prompt_move(&self.state) { break uci_mv }
          }
        },
        Computer { engine } => {
          let mv = engine.best_move(&self.state);
          let uci_mv: UciMove = move.into();
          let msg = UciMessage::best_move(uci_mv);
          println!("{}", msg.serialize());
          uci_mv
        }
      };
      if let Player::Computer { .. } = playing_against {
        let position_msg = {
          let fen: UciFen = self.state.fen_string().into();
          UciMessage::Position { startpos: false, fen, moves: vec![uci_mv] }
        };
        println!("{}", position_msg);
      };
      self.state.execute(uci_mv.into());
    }
  }
}
