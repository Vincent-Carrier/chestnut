use core::sq::SqSize;
use std::io::stdin;
use core::state::State;
use core::sq::Sq;
use core::moves::*;
use regex::Regex;


pub trait UI {
  fn prompt_move(&self, s: &State) -> Option<Move>;
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

  fn prompt_move(&self, s: &State) -> Option<Move> {
    println!("Please enter your move: ");
    let mut move_str = String::new();
    stdin().read_line(&mut move_str).unwrap();
    let mv = parse_move(&move_str, s).unwrap();
    Some(mv)
  }
}

static SQ: &str    = r"([a-h][1-8])";
static PIECE: &str = r"([BNRQK])";
lazy_static! {
  static ref PAWN: Regex = Regex::new(SQ).unwrap();
  static ref PAWN_VERBOSE: Regex = Regex::new(&format!("{}-{}", SQ, SQ)).unwrap();
  static ref PAWN_CAPTURE: Regex = Regex::new(&format!("^{}x{}$", SQ, SQ)).unwrap();
  static ref NORMAL: Regex = Regex::new(&format!("^{}-{}$", PIECE, SQ)).unwrap();
  static ref NORMAL_VERBOSE: Regex = Regex::new(&format!("^{}{}-{}$", PIECE, SQ, SQ)).unwrap();
  static ref CAPTURE: Regex = Regex::new(&format!("^{}x{}$", PIECE, SQ)).unwrap();
  static ref CAPTURE_VERBOSE: Regex = Regex::new(&format!("^{}{}x{}$", PIECE, SQ, SQ)).unwrap();
}

fn parse_sq(input: &str) -> Sq {
  let chars = input.as_bytes();
  Sq {
    x: chars[0] as SqSize - 97,
    y: 56 - chars[1] as SqSize
  }
}

fn parse_move(input: &str, s: &State) -> Result<Move, &'static str> {
  let mv = match input {
    "O-O" => Move::Castle { side: Side::King },
    "O-O-O" => Move::Castle { side: Side::Queen },
    _ => {
      if PAWN_VERBOSE.is_match(&input) {
        Move::Normal {
          from: parse_sq(&input[..2]),
          to: parse_sq(&input[3..]),
          capture: None,
        }
      } else {
        panic!("Notation not implemented")
      }
    }
  };
  Ok(mv)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse_a_sq() {
    assert_eq!(
      parse_sq("a8"),
      Sq { x: 0, y: 0 }
    );
    assert_eq!(
      parse_sq("h7"),
      Sq { x: 7, y: 1 }
    );
    assert_eq!(
      parse_sq("e4"),
      Sq { x: 4, y: 4 }
    );
  }
}

