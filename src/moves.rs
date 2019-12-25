use crate::piece::Piece;
use crate::sq::*;
use crate::color::{*, Color::*};
use crate::board::*;
use crate::state::*;

pub trait Move {
  fn valid(&self, s: &State) -> bool;
  fn execute(&self, s: &State) -> Board;
  fn undo(&self, s: &State) -> Board;
}

pub struct NormalMove {
  from: Sq, to: Sq
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
  }

  fn undo(&self, s: &State) -> Board {

  }
}

pub struct CaptureMove {
  mv: NormalMove, capture: Piece
}

enum Side { Queen, King }

pub struct CastleMove {
  side: Side
}

pub struct EnPassantMove {
  mv: NormalMove, capture: Piece
}
