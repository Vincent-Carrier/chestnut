use crate::piece::Piece;
use crate::sq::*;
use crate::board::*;
use crate::state::*;

#[derive(Clone, Copy)]
pub enum Side { Queen, King }

#[derive(Clone, Copy)]
pub enum Move {
  Normal    { piece: Piece, from: Sq, to: Sq, capture: Option<Piece> },
  Castle    { piece: Piece, side: Side },
  EnPassant { piece: Piece, },
}

pub use self::Move::*;

impl Move {
  pub fn valid (&self, s: &State) -> bool {
    match self {
      Normal { piece, from, to, capture } => {
        match s.board[*from] {
          Some(p) => {
            p.color == s.active_color && *capture == s.board[*to]
          },
          None => false
        }
      },
      Castle { side, .. }=> {
        s.can_castle(s.active_color, *side)
      },
      EnPassant { piece } => {
        false
      },
    }
  }

  pub fn execute(&self, s: &State) -> Board {
    let mut board = s.board.clone();
    match self {
      Normal { from, to, piece, .. } => {
        board[to] = Some(piece);
        board[from] = None;
      },
      EnPassant { piece } => {
        panic!("Not implemented")
      },
      Castle { side, piece } => {
        panic!("Not implemented")
      },
    }
    board
  }

  pub fn undo (&self, s: &State) -> Board {
    panic!("Not implemented")
  }
}


// fn undo(&self, s: &State) -> Board {
//   let mv = NormalMove { from: self.to, to: self.from };
//   mv.execute(s)
// }

