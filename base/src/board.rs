use crate::sq::*;
use crate::color::Color;
use crate::piece::*;
use crate::board_iter;
use crate::moves::*;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;


#[derive(Clone, Default)]
pub struct Board {
  pieces: [[Option<Piece>; 8]; 8],
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut output = String::new();
    for (sq, content) in self.iter() {
      let ch = match content {
        Some(p) => p.unicode_char(),
        _ => ' '
      };
      output.push(ch);
      if sq.x == 7 { output.push('\n'); }
    }
    write!(f, "{}", output)
  }
}

impl std::ops::Index<Sq> for Board {
  type Output = Option<Piece>;

  fn index(&self, sq: Sq) -> &Self::Output {
    &self.pieces[sq.y as usize][sq.x as usize]
  }
}

impl std::ops::IndexMut<Sq> for Board {
  fn index_mut(&mut self, sq: Sq) -> &mut Self::Output {
    &mut self.pieces[sq.y as usize][sq.x as usize]
  }
}

impl Board {
  pub fn new() -> Board {
    Board::default()
  }

  pub fn iter(&self) -> board_iter::Iter {
    board_iter::Iter { counter: Sq { x: -1, y: 0 }, board: self }
  }

  pub fn pieces_of(&self, color: Color) -> impl Iterator<Item = (Sq, Piece)> + '_ {
    self.iter().filter_map(
      |(sq, content)| match content {
        Some(piece) if piece.color == color => Some((sq, piece)),
        _ => None
      }
  }

  pub fn range_of(&self, color: Color) -> impl Iterator<Item = Sq> + '_ {
    self.pieces_of(color).flat_map(move |(from, piece)| {
      piece.moves(from, self)
    })
  }

  pub fn moves_of(&self, color: Color) -> impl Iterator<Item = Move> + '_ {
    self.pieces_of(color).flat_map(move |(from, piece)| {
      piece.moves(from, self).into_iter()
        .map(move |to| Move::Normal { from, to, piece, capture: self[to] })
    })
  }

  pub fn is_threatened(&self, sq: Sq, by: Color) -> bool {
    self.range_of(by).any(|threat_sq| threat_sq == sq)
  }

  pub fn from_file(file: &'static str) -> Board {
    let mut f = File::open(file).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");
    let mut result = Board::new();
    for (y, line) in contents.lines().enumerate() {
      for (x, ch) in line.chars().enumerate() {
        if ch != '.' {
          let sq = Sq { x: x as SqSize, y: y as SqSize };
          result[sq] = Some(ch.into())
        }
      }
    }
    result
  }
}

lazy_static! {
  pub static ref INITIAL_BOARD: Board = Board::from_file("boards/initial.txt");
}
