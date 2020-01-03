use core::prelude::*;

pub struct UserMove {
  from: Sq, to: Sq
}

impl UserMove {
  pub fn validate(&self, s: &State) -> Result<Move, &str> {
    let from = s.board[self.from]?;
    let capture = s.board[self.to];
    Err("sorry")
  }
}

// fn castle_sq(color: Color, side: Side) -> Sq {
  // Sq { x: if let side::Queen { 2 } else { 6 }, y: color.home_row() }
// }
// 
// fn king_sq(color: Color) -> {
  // Sq { x: 4, y: color.home_row() }
// }
