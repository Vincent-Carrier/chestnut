use crate::sq::SqSize;
pub use self::Color::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
  White, Black,
}


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

}
