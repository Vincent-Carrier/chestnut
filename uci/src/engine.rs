use crate::bridge::from_move;
use crate::bridge::into_move;
use crate::fen::uci_fen;
use vampirc_uci::UciFen;
use std::io::prelude::*;
use base::prelude::*;
use std::io::{stdin,BufReader};
use base::player::Player;
use vampirc_uci::parse;
use vampirc_uci::uci::UciMessage;

#[derive(Default)]
pub struct UciEngine {}

impl Player for UciEngine {
  fn accept_move(&self, mv: &Move, state: &State) {
    let position_msg = {
      let fen: Option<UciFen> = Some(uci_fen(state));
      UciMessage::Position { startpos: false, fen, moves: vec![into_move(*mv)] }
    };
    println!("{}", position_msg);
  }

  fn post_move(&self, state: &State) -> Move {
    let reader = BufReader::new(stdin());
    let mut lines = reader.lines();
    loop {
      let line = lines.next().unwrap().unwrap();
      let messages = parse(&line);
      match messages.get(0).unwrap() {
        UciMessage::BestMove { best_move, .. } => break from_move(*best_move, &state.board),
        _ => continue
      };
    }
  }
}
