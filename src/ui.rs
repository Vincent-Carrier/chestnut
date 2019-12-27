use std::io::stdin;
use crate::state::State;
use crate::sq::Sq;
use crate::moves::*;
use regex::Regex;


pub trait UI {
  fn prompt_move(&self, s: &State) -> Option<MovePtr>;
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
    println!("It's {:?}'s turn to play!", s.active_player.color);
  }

  fn prompt_move(&self, s: &State) -> Option<MovePtr> {
    println!("Please enter your move: ");
    let mut move_str = String::new();
    stdin().read_line(&mut move_str).unwrap();
    let mv = parse_move(move_str, s).unwrap();
    Some(mv)
  }
}

static SQ: &str    = r"([a-h][1-8])";
static PIECE: &str = r"([BNRQK])";
lazy_static! {
  static ref PAWN: Regex = Regex::new(SQ).unwrap();
  static ref PAWN_VERBOSE: Regex = Regex::new(&format!("^{}-{}$", SQ, SQ)).unwrap();
  static ref PAWN_CAPTURE: Regex = Regex::new(&format!("^{}x{}$", SQ, SQ)).unwrap();
  static ref NORMAL: Regex = Regex::new(&format!("^{}-{}$", PIECE, SQ)).unwrap();
  static ref NORMAL_VERBOSE: Regex = Regex::new(&format!("^{}{}-{}$", PIECE, SQ, SQ)).unwrap();
  static ref CAPTURE: Regex = Regex::new(&format!("^{}x{}$", PIECE, SQ)).unwrap();
  static ref CAPTURE_VERBOSE: Regex = Regex::new(&format!("^{}{}x{}$", PIECE, SQ, SQ)).unwrap();
}

fn parse_sq(input: &str) -> Sq {
  let chars = input.as_bytes();
  Sq {
    x: chars[0] as i32 - 97,
    y: 56 - chars[1] as i32
  }
}

fn parse_move(input: String, s: &State) -> Result<MovePtr, &'static str> {
  let mv: MovePtr = match input {
    // "O-O" => Box::new(CastleMove { side: Side::King }),
    // "O-O-O" => Box::new(CastleMove { side: Side::Queen }),
    _ => {
      if PAWN_VERBOSE.is_match(&input) {
        Box::new(NormalMove { from: parse_sq(&input[0..2]), to: parse_sq(&input[3..5]) })
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

