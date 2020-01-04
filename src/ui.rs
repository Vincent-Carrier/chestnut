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

fn parse_move(s: String) -> UiResult {
  if !MOVE_REGEX.is_match(&s) { return Err("couldn't parse your move") }
  let caps = MOVE_REGEX.captures(&s).unwrap();
  // TODO: Promotion
  let mv = UciMove {
    from: sq_from_str(caps.get(1).unwrap().as_str()),
    to: sq_from_str(caps.get(2).unwrap().as_str()),
    promotion: None,
  };
  Ok(mv)
}

fn sq_from_str(s: &str) -> UciSquare {
  let mut chars = s.chars();
  UciSquare {
    file: chars.next().unwrap(),
    rank: chars.next().unwrap().to_digit(10).unwrap() as u8,
  }
}


lazy_static! {
  static ref MOVE_REGEX: Regex = Regex::new(r"([a-h][1-8])([a-h][1-8])([nbrq])?").unwrap();
}
