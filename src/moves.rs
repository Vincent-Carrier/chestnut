use crate::piece::Piece;
use crate::sq::*;
use crate::board::*;
use crate::state::*;

pub trait Move {
  fn valid(&self, s: &State) -> bool;
  fn execute(&self, s: &State) -> Board;
  fn undo(&self, s: &State) -> Board;
}

pub type MovePtr = Box<dyn Move>;

pub struct NormalMove {
  pub from: Sq, pub to: Sq
}

impl Move for NormalMove {
  fn valid(&self, s: &State) -> bool {
    let active_color = s.active_player.color;
    match s.board.at(self.from) {
      Some(p) => p.color == active_color && s.board.at(self.to).is_none(),
      _ => false
    }
  }

  fn execute(&self, s: &State) -> Board {
    let mut board = s.board.clone();
    let piece = board.piece_at(self.from);
    board.set_at(self.to, piece);
    board.clear(self.from);
    board
  }

  fn undo(&self, s: &State) -> Board {
    let mv = NormalMove { from: self.to, to: self.from };
    mv.execute(s)
  }
}

pub struct CaptureMove {
  mv: NormalMove, capture: Piece
}

pub enum Side { Queen, King }

pub struct CastleMove {
  side: Side
}

pub struct EnPassantMove {
  mv: NormalMove, capture: Piece
}
