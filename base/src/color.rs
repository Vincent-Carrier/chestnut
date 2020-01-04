use crate::sq::SqSize;
use strum_macros::{Display, EnumIter};

#[derive(Display, EnumIter, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Color {
  White,
  Black,
}

pub use self::Color::*;

impl Color {
  pub fn opposite(self) -> Color {
    if self == White { Black } else { White }
  }

  pub fn pawn_row(self) -> SqSize {
    if self == White { 6 } else { 1 }
  }

  pub fn home_row(self) -> SqSize {
    if self == White { 7 } else { 0 }
  }

  pub fn en_passant_row(self) -> SqSize {
    if self == White { 3 } else { 4 }
  }

  pub fn forward(self) -> SqSize {
    if self == White { -1 } else { 1 }
  }

  pub fn char(self) -> char { if self == White { 'w' } else { 'b' } }
}

impl From<char> for Color {
  fn from(ch: char) -> Color {
    if ch.is_ascii_uppercase() { White } else { Black }
  }
}
