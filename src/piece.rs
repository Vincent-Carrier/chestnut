use crate::color::{*, Color::*};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Piece {
  pub kind: PieceKind,
  pub color: Color
}

impl Piece {
  pub fn char(&self) -> char {
    let ch = self.kind.char();
    if self.color == White { ch } else { ch.to_ascii_lowercase() }
  }
}
