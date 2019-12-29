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
  pub fn valid  (&self, s: &State) -> bool {
    match self {
      Normal { piece, from, to, capture } => {
        match s.board.at(*from) {
          Some(p) => {
            p.color == s.active_color &&
            capture.as_ref() == s.board.at(*to)
          },
          None => false
        }
      },
      Castle { side, .. } => {
        s.can_castle()
      },
      EnPassant  { piece }=> {
        false
      },
    }
  }

  pub fn execute(&self, s: &State) -> Board {
    let mut board = s.board.clone();
    match self {
      Normal { from, to, .. } => {
        let piece = board.piece_at(*from);
        board.set_at(*to, piece);
        board.clear(*from);
      },
      EnPassant => {
        panic!("Not implemented")
      },
      Castle { side } => {
        panic!("Not implemented")
      },
    }
    board
  }

  pub fn undo   (&self, s: &State) -> Board {
    panic!("Not implemented")
  }
}


fn valid(from: Sq, to: Sq, State { board, active_color, .. }: &State) -> bool {
}
// fn undo(&self, s: &State) -> Board {
//   let mv = NormalMove { from: self.to, to: self.from };
//   mv.execute(s)
// }

