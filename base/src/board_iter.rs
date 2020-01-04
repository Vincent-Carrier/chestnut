use crate::moves::Move;
use crate::color::Color;
use crate::piece::Piece;
use crate::board::Board;
use crate::sq::Sq;

pub struct Iter<'a> {
  pub counter: Sq,
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

impl Board {
  pub fn iter(&self) -> Iter {
    Iter { counter: Sq { x: -1, y: 0 }, board: self }
  }

  pub fn pieces_of(&self, color: Color) -> impl Iterator<Item = (Sq, Piece)> + '_ {
    self.iter().filter_map(
      |(sq, content)| match content {
        Some(piece) if piece.color == color => Some((sq, piece)),
        _ => None
      }
    )
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
}


