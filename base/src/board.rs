use crate::sq::*;
use crate::piece::*;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;


#[derive(Clone)]
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

impl Default for Board {
  fn default() -> Self { INITIAL_BOARD.clone() }
}

impl Board {
  pub fn empty() -> Board { Board { pieces: [[None; 8]; 8] } }

  pub fn new() -> Board { Board::default() }

  pub fn from_file(file: &'static str) -> Board {
    let mut f = File::open(file).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");
    let mut result = Board::empty();
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
  pub static ref INITIAL_BOARD: Board = Board::from_file("base/boards/initial.txt");
}
