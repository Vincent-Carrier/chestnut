use std::io::prelude::*;
use base::prelude::Color::{White,Black};
use crate::player::Player::Computer;
use crate::player::Player::Human;
use std::collections::BTreeMap;
use crate::player::Player;
use crate::ui::CLI;
use engine::random_engine::RandomEngine;
use base::prelude::*;
use base::prelude::KingState::{Checkmate};
use vampirc_uci::{UciMessage, UciFen, Serializable};

pub struct Game {
  state: State,
  players: BTreeMap<Color, Player>
}

impl Game {
  pub fn new() -> Game {
    let mut players = BTreeMap::new();
    let engine = RandomEngine::new();
    players.insert(White, Human { ui: CLI::new() });
    players.insert(Black, Computer { engine: Box::from(engine) });
    Game { state: State::new(), players }
  }

  pub fn start(&mut self) {
    while self.state.king_state != Checkmate {
      let player = self.players[&self.state.active_color];
      let playing_against = self.players[&self.state.active_color.opposite()];
      let mv = match player {
        Human { ui } => {
          ui.prompt_turn(&self.state);
          loop {
            if let Ok(mv) = ui.prompt_move(&self.state) { break mv }
          }
        },
        Computer { engine } => {
          let mv = engine.best_move(&self.state);
          // let mv: UciMove = mv.into();
          // let msg = UciMessage::best_move(mv);
          // println!("{}", msg.serialize());
          mv
        },
        Uci { reader } => {
          for line in reader.lines() {
            let messages = vampirc_uci::parse(line) {
              BestMove()
            }
          }
        }
      };

      if let Player::Uci { .. } = playing_against {
        let position_msg = {
          let fen: Option<UciFen> = Some(self.state.into());
          UciMessage::Position { startpos: false, fen, moves: vec![mv.into()] }
        };
        println!("{}", position_msg);
      };

      self.state.execute(mv);
    }
  }
}
