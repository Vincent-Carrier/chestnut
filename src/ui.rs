use base::uci_bridge::parse_move;
use std::io::stdin;
use base::prelude::*;

type UiResult = Result<Move, &'static str>;

pub trait UI {
  fn prompt_move(&self, s: &State) -> UiResult;
  fn prompt_turn(&self, s: &State);
}

pub struct CLI {}

impl CLI {
  pub fn new() -> Box<dyn UI> {
    Box::new(CLI {})
  }
}

impl UI for CLI {
  fn prompt_turn(&self, s: &State) {
    println!("{}", s.board);
    println!("It's {:?}'s turn to play!", s.active_color);
  }

  fn prompt_move(&self, s: &State) -> UiResult {
    println!("Please enter your move: ");
    let mut move_str = String::new();
    stdin().read_line(&mut move_str).unwrap();
    let mv = parse_move(move_str, &s.board);
    Ok(mv)
  }
}





