#![allow(overflowing_literals)]

use crate::color::Color;
use crate::sq::*;
use crate::color::Color::*;
use crate::piece::{*, PieceKind::*};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

struct Iter<'a> {
  counter: Sq,
  board: &'a Board
}

#[derive(Clone, Default)]
pub struct Board {
  pieces: [[Option<Piece>; 8]; 8],
}

pub type PieceList = Vec<(Sq, Piece)>;

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


impl Iterator for Iter<'_> {
  type Item = (Sq, Option<Piece>);

  fn next(&mut self) -> Option<Self::Item> {
    let counter = self.counter;
    if counter.x == 7 {
      counter.y += 1;
    } else {
      counter.x += 1;
    }
    if counter.y > 7 { None } else {
      let sq = Sq { x: counter.x, y: counter.y };
      Some((sq, self.board[sq]))
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (64, Some(64))
  }
}

impl Board {
  // pub fn at(&self, sq: Sq) -> Option<Piece> {
    // self[sq]
  // }
// 
  // pub fn piece_at(&self, sq: Sq) -> Piece {
    // self.at(sq).unwrap()
  // }
// 
  // pub fn set_at(&mut self, sq: Sq, piece: Piece) {
    // self[sq] = Some(piece);
  // }
// 
  // pub fn clear(&mut self, sq: Sq) {
    // self[sq] = None;
  // }
// 
  pub fn new() -> Board {
    Default::default()
  }

  pub fn iter(&self) -> Iter {
    Iter { counter: Sq { x: -1, y: 0 }, board: self }
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
          let sq = Sq { x: x as SqSize, y: y as SqSize };
          result[sq] = Some(Piece { color, kind })
        }
      }
    }
    result
  }

  pub fn pieces_of(&self, color: Color) -> PieceList {
    self.iter().filter_map(|(sq, content)| match content {
      Some(p) if p.color == color => Some((sq, p)),
      _ => None
    }).collect()
  }
}

lazy_static! {
  pub static ref INITIAL_BOARD: Board = Board::from_file("boards/initial.txt");
}
