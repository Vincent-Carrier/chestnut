use crate::color::*;
use std::convert::TryFrom;

#[derive(Clone, Copy)]
pub enum PieceKind {
  Pawn, Knight, Bishop, Rook, Queen, King
}

use crate::piece::PieceKind::*;

impl PieceKind {
  pub fn char(self) -> char {
    match self {
      Pawn   => 'P',
      Knight => 'N',
      Bishop => 'B',
      Rook   => 'R',
      Queen  => 'Q',
      King   => 'K',
    }
  }

  pub fn unicode_char(self) -> char {
    match self {
      Pawn   => '♙',
      Knight => '♘',
      Bishop => '♗',
      Rook   => '♖',
      Queen  => '♕',
      King   => '♔',
    }
  }
}

#[derive(Clone, Copy)]
pub struct Piece {
  pub kind: PieceKind,
  pub color: Color
}

impl Piece {
  pub fn char(self) -> char {
    let ch = self.kind.char();
    if self.color == White { ch } else { ch.to_ascii_lowercase() }
  }

  pub fn unicode_char(self) -> char {
    let ch = self.kind.unicode_char();
    if self.color == White { ch } else {
      let code_pt = u32::try_from(ch).unwrap() + 6;
      std::char::from_u32(code_pt).unwrap()
    }
  }
}
