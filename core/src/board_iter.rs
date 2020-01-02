use crate::color::Color;
use crate::piece::Piece;
use crate::board::Board;
use crate::sq::Sq;

pub struct Iter<'a> {
  counter: Sq,
  board: &'a Board
}

impl Iterator for Iter<'_> {
  type Item = (Sq, Option<Piece>);

  fn next(&mut self) -> Option<Self::Item> {
    let mut counter = self.counter;
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

pub struct PieceIter<'a> {
  counter: Sq,
  board: &'a Board,
  color: Color,
}

impl Iterator for PieceIter<'_> {
  type Item = (Sq, Piece);

  fn next(&mut self) -> Option<Self::Item> {
    let mut counter = self.counter;
    if counter.x == 7 {
      counter.y += 1;
    } else {
      counter.x += 1;
    }
    if counter.y > 7 { None } else {
      let sq = Sq { x: counter.x, y: counter.y };
      if let Some(p) = self.board[sq] {
        if p.color == self.color {
          Some((sq, p))
        } else { None }
      } else { None }
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (1, Some(16))
  }
}

