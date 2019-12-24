#[derive(PartialEq, Clone, Copy)]
pub enum Color {
  White, Black,
}

use crate::color::Color::*;

impl Color {
  pub fn opposite(self) -> Color {
    if self == White { Black } else { White }
  }

  pub fn pawn_row(self) -> i32 {
    if self == White { 6 } else { 1 }
  }

  pub fn home_row(self) -> i32 {
    if self == White { 7 } else { 0 }
  }

  pub fn forward(self) -> i32 {
    if self == White { -1 } else { 1 }
  }
}
