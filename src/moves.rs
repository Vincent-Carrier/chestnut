use crate::piece::Piece;
use crate::sq::*;
use crate::board::*;
use crate::state::*;

#[derive(Clone, Copy)]
pub enum Side { Queen, King }

#[derive(Clone, Copy)]
pub enum Move {
  Normal    { from: Sq, to: Sq },
  Capture   { from: Sq, to: Sq, capture: Piece },
  EnPassant { from: Sq, to: Sq },
  Castle    { side: Side },
}

pub use self::Move::*;

impl Move {
  pub fn valid  (&self, s: &State) -> bool {
    match self {
      Normal { from, to } => {
        valid(*from, *to, s)
      },
      Capture { from, to, capture } => {
        false
      },
      EnPassant { from, to } => {
        false
      },
      Castle { side } => {
        false
      },
    }
  }

  pub fn execute(&self, s: &State) -> Board {
    match self {
      Normal { from, to } => {
        let mut board = s.board.clone();
        let piece = board.piece_at(*from);
        board.set_at(*to, piece);
        board.clear(*from);
        board
      },
      Capture { from, to, capture } => {
        panic!("Not implemented")
      },
      EnPassant { from, to } => {
        panic!("Not implemented")
      },
      Castle { side } => {
        panic!("Not implemented")
      },
    }
  }

  pub fn undo   (&self, s: &State) -> Board {
    panic!("Not implemented")
  }
}


fn valid(from: Sq, to: Sq, State { board, active_color, .. }: &State) -> bool {
  match board.at(from) {
    Some(p) => p.color == *active_color && board.at(to).is_none(),
    _ => false
  }
}
// fn undo(&self, s: &State) -> Board {
//   let mv = NormalMove { from: self.to, to: self.from };
//   mv.execute(s)
// }

