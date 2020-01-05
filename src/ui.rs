use vampirc_uci::UciSquare;
use vampirc_uci::UciMove;
use base::sq::SqSize;
use std::io::stdin;
use base::state::State;
use base::sq::Sq;
use base::moves::*;
use vampirc_uci::parse_strict;
use regex::Regex;

type UiResult = Result<UciMove, &'static str>;

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
    let mv = parse_move(move_str);
    parse_move(move_str)
  }
}





