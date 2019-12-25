use crate::color::*;

#[derive(Clone)]
pub enum PieceKind {
  Pawn, Knight, Bishop, Rook, Queen, King
}

use crate::piece::PieceKind::*;

impl PieceKind {
  pub fn char(&self) -> char {
    match self {
      Pawn   => 'P',
      Knight => 'N',
      Bishop => 'B',
      Rook   => 'R',
      Queen  => 'Q',
      King   => 'K',
    }
  }

}

#[derive(Clone)]
pub struct Piece {
  pub kind: PieceKind,
  pub color: Color
}
