use crate::state::State;
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

impl State {
  pub fn fen_string(&self) -> String {
    let iter = self.board.iter();
    let mut string = String::with_capacity(64);
    let mut empty_sq_count = 0;

    let push_empty_count = ||
        if empty_sq_count > 0 {
          string.push(std::char::from_digit(empty_sq_count, 10).unwrap());
          empty_sq_count = 0;
        };

    while let Some((sq, content)) = iter.next() {
      if let Some(piece) = content {
        push_empty_count();
        string.push(piece.char());
      } else {
        empty_sq_count += 1;
      }
      if iter.counter.x == 7 {
        string.push('/');
        empty_sq_count = 0;
      }
    };

    // TODO: Castling, EnPassant, half-move, full-move
    
    string
  }
}
