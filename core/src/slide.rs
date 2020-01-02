use crate::sq::{Vec2, Sq, SqSize, four_directions};
use crate::board::*;
use crate::piece::*;
use crate::piece::PieceKind::*;

lazy_static! {
  static ref DIAGONALS: Vec<Vec2> =
    four_directions(Vec2 { x: 1, y: 1 }).collect();
 static ref STRAIGHT_LINES: Vec<Vec2> =
    four_directions(Vec2 { x: 0, y: 1 }).collect();
  static ref EIGHT_DIRECTIONS: Vec<Vec2> =
    four_directions(Vec2 { x: 1, y: 1 }).chain(four_directions(Vec2 { x: 0, y: 1 })).collect();
  static ref KNIGHT_DIRECTIONS: Vec<Vec2> =
    four_directions(Vec2 { x: 1, y: 2 }).chain(four_directions(Vec2 { x: 2, y: 1 })).collect();
}

#[derive(PartialEq)]
enum CaptureStyle {
  Can, Cannot, Must
}

use self::CaptureStyle::*;

struct SlideIter<'a> {
  board: &'a Board,
  piece: Piece,
  from: Sq,
  dir: Vec2,
  capture: CaptureStyle,
  i: SqSize,
  max: SqSize
}

impl Iterator for SlideIter<'_> {
  type Item = Sq;

  fn next(&mut self) -> Option<Sq> {
    if self.i >= self.max { return None }
    self.i += 1;
    let sq = self.from + (self.dir * self.i);
    if !sq.inside_board() { return None };

    match self.board[sq] {
      None if self.capture != Must => Some(sq),
      Some(p) if p.color == self.piece.color => { self.i = self.max; Some(sq) },
      Some(p) if p.color != self.piece.color && self.capture != Cannot => Some(sq),
      _ => None
    }
  }
}

impl Piece {
  pub fn moves(&self, from: Sq, board: &Board) -> Vec<Sq> {
    let slide =
      move |dir, max, capture| SlideIter { board, from, dir, piece: self, capture, i: 0, max };

    if self.kind == Pawn {
      let forward_dist = if from.y == self.color.pawn_row() { 2 } else { 1 };
      let forward_dir = Vec2 { x: 0, y: self.color.forward() };
      let capture_dirs =
        [Vec2 { x: -1, y: self.color.forward() }, Vec2 { x: 1, y: self.color.forward() }];

      return capture_dirs
        .iter()
        .flat_map(|dir| slide(*dir, 1, Must))
        .chain(slide(forward_dir, forward_dist, Cannot))
        .collect();
    }

    let dirs = match self.kind {
      Knight => *KNIGHT_DIRECTIONS,
      Bishop => *DIAGONALS,
      Rook => *STRAIGHT_LINES,
      King | Queen => *EIGHT_DIRECTIONS,
    };

    dirs
      .iter()
      .flat_map(|dir| {
        let max = match self.kind {
          Knight | King => 1,
          _ => 8,
        };
        slide(*dir, max, Can)
      })
      .collect()
  }
}

pub fn display_moves(moves: &[Vec2]) {
  for y in 0..8 {
    for x in 0..8 {
      if moves.contains(&Sq { x: x as SqSize, y: y as SqSize }) {
        print!("X");
      } else {
        print!(".")
      }
    }
    println!();
  }
}
