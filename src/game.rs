use std::io::prelude::*;
use std::io::BufReader;
use std::io::stdin;
use base::prelude::Color::{White,Black};
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
    let uci = Player::Uci { reader: BufReader::new(stdin()) };
    players.insert(Black, uci);
    players.insert(White, Player::Computer { engine: Box::from(engine) });
    Game { state: State::new(), players }
  }

  pub fn start(&mut self) {
    while self.state.king_state != Checkmate {
      let player = self.players[&self.state.active_color];
      let playing_against = self.players[&self.state.active_color.opposite()];
      let mv = match player {
        Player::Human { ui } => {
          ui.prompt_turn(&self.state);
          loop {
            if let Ok(mv) = ui.prompt_move(&self.state) { break mv }
          }
        },
        Player::Computer { engine } => engine.best_move(&self.state),
        Player::Uci { reader } => {
          let messages = vampirc_uci::parse(reader.read_line());
          loop {
            match messages.iter().next() {
              UciMessage::BestMove { best_move, .. } => break best_move.into(),
              _ => ()
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
