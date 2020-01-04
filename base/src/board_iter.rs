use crate::moves::Move;
use crate::color::Color;
use crate::piece::Piece;
use crate::board::Board;
use crate::sq::Sq;
use itertools::iproduct;

impl Board {
  pub fn iter(&self) -> impl Iterator<Item = (Sq, Option<Piece>)> + '_ {
    iproduct!(0..8, 0..8).map(|(x, y)| {
      let sq = Sq { x, y };
      (sq, self[sq])
    })
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


