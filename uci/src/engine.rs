use vampirc_uci::UciFen;
use std::io::BufRead;
use base::prelude::*;
use std::io::stdout;
use std::io::stdin;
use base::player::Player;
use std::io::Stdout;
use std::io::Stdin;
use std::io::BufWriter;
use std::io::BufReader;
use vampirc_uci::parse;
use vampirc_uci::uci::UciMessage;

struct UciEngine {
  reader: BufReader<Stdin>,
  writer: BufWriter<Stdout>,
}

impl Default for UciEngine {
  fn default() -> Self {
    UciEngine {
      reader: BufReader::new(stdin()),
      writer: BufWriter::new(stdout()),
    }
  }
}

impl Player for UciEngine {
  fn game_start(&self, state: &State) {

  }

  fn prompt_turn(&self, state: &State) {
  }

  fn accept_move(&self, mv: &Move) {
    let position_msg = {
      let fen: Option<UciFen> = Some(state.into());
      UciMessage::Position { startpos: false, fen, moves: vec![mv.into()] }
    };
    println!("{}", position_msg);
  }

  fn post_move(&self, state: &State) -> Move {
    let lines = self.reader.lines();
    loop {
      let line = lines.next().unwrap().unwrap();
      match parse(&line).get(0).unwrap() {
        UciMessage::BestMove { best_move, .. } => break best_move.into(),
        _ => continue
      };
    }
  }
}
