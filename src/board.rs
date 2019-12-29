#![allow(overflowing_literals)]

use crate::sq::*;
use crate::color::Color::*;
use crate::piece::{*, PieceKind::*};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Board {
  pieces: [[Option<Piece>; 8]; 8]
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut output = String::new();
    for y in 0..8 {
      for x in 0..8 {
        let content = self.at(Sq { x, y });
        let ch = match content {
          Some(p) => p.unicode_char(),
          None if (x + y) % 2 == 0 => ' ',
          _ => '◯'
        };
        output.push(ch);
      }
      output.push('\n');
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
    &mut self[sq]
  }
}

impl Board {
  pub fn at(&self, sq: Sq) -> Option<Piece> {
    self[sq]
  }

  pub fn piece_at(&self, sq: Sq) -> Piece {
    self.at(sq).unwrap()
  }

  pub fn set_at(&mut self, sq: Sq, piece: Piece) {
    self[sq] = Some(piece);
  }

  pub fn clear(&mut self, sq: Sq) {
    self[sq] = None;
  }

  pub fn new() -> Board {
    Board { pieces: [[None; 8]; 8] }
  }

  pub fn from_file(file: &'static str) -> Board {
    let mut f = File::open(file).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");
    let mut result = Board::new();
    for (y, line) in contents.lines().enumerate() {
      for (x, ch) in line.chars().enumerate() {
        if ch != '.' {
          let color = if ch.is_ascii_uppercase() { White } else { Black };
          let kind = match ch.to_ascii_uppercase() {
            'P' => Pawn,
            'N' => Knight,
            'B' => Bishop,
            'R' => Rook,
            'Q' => Queen,
            'K' => King,
             _  => panic!("Unexpected character")
          };
          result.set_at(
            Sq { x: x as SqSize, y: y as SqSize },
            Piece { color, kind }
          )
        }
      }
    }
    result
  }
}

lazy_static! {
  pub static ref INITIAL_BOARD: Board = Board::from_file("boards/initial.txt");
}
