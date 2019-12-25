use std::ops;
use crate::sq::*;
use crate::color::*;
use crate::board::*;
use crate::piece::*;
use crate::piece::PieceKind::*;


impl ops::Add<Vec2> for Vec2 {
  type Output = Vec2;
  fn add(self, _rhs: Vec2) -> Vec2 {
    Vec2 { x: self.x + _rhs.x, y: self.y + _rhs.y }
  }
}

impl Vec2 {
  fn scale(self, n: i32) -> Vec2 {
    Vec2 { x: self.x * n, y: self.y * n }
  }

  const fn rotate(self) -> Vec2 {
    Vec2 { x: -self.y, y: self.x }
  }

  fn inside_board(self) -> bool {
    0 <= self.x && self.x <= 7 &&
    0 <= self.y && self.y <= 7
  }
}

// Rust's lack of compile-time loops is infuriating
const fn four_directions(dir: Vec2) -> [Vec2; 4] {
  [ dir, dir.rotate(), dir.rotate().rotate(), dir.rotate().rotate().rotate() ]
}

const fn eight_directions(dir: Vec2) -> [Vec2; 8] {
  let alt = Vec2 { x: 0, y: dir.y };
  [ dir, dir.rotate(), dir.rotate().rotate(), dir.rotate().rotate().rotate() ,
    alt, alt.rotate(), alt.rotate().rotate(), alt.rotate().rotate().rotate() ]
}

const fn knight_directions(dir: Vec2) -> [Vec2; 8] {
  let inv = Vec2 { x: dir.y, y: dir.x };
  [ dir, dir.rotate(), dir.rotate().rotate(), dir.rotate().rotate().rotate() ,
    inv, inv.rotate(), inv.rotate().rotate(), inv.rotate().rotate().rotate() ]
}

static DIAGONALS        : [Vec2; 4] = four_directions(Vec2 { x: 1, y: 1 });
static STRAIGHT_LINES   : [Vec2; 4] = four_directions(Vec2 { x: 0, y: 1 });
static KNIGHT_DIRECTIONS: [Vec2; 8] = knight_directions(Vec2 { x: 1, y: 2 });
static EIGHT_DIRECTIONS : [Vec2; 8] = eight_directions(Vec2 { x: 1, y: 1 });

#[derive(PartialEq)]
enum CaptureStyle {
  Can, Cannot, Must
}

use crate::movement::CaptureStyle::*;

fn slide(from: Sq, towards: Vec2, color: Color, board: &Board,
         max: i32, capture_style: CaptureStyle) -> Vec<Sq> {
  let mut sqs: Vec<Sq> = Vec::new();
  let mut i = 1;
  loop {
    let candidate = from + towards.scale(i);
    if candidate.inside_board() && i <= max {
      match board.at(candidate) {
        None if capture_style == Must => return sqs,
        None => sqs.push(candidate),
        Some(p) if p.color == color || capture_style == Cannot => return sqs,
        Some(_) => { sqs.push(candidate); return sqs },
      }
    } else {
      return sqs
    }
    i += 1;
  }
}


impl Piece {
  pub fn moves(&self, from: Sq, board: &Board) -> Vec<Sq> {
    match &self.kind {
      Pawn   => {
        let forward_dist = if from.y == self.color.pawn_row() { 2 } else { 1 };
        let forward_dir = Vec2 { x: 0, y: self.color.forward() };
        let capture_dirs = vec![
          Vec2 { x: -1, y: self.color.forward() },
          Vec2 { x: 1, y: self.color.forward() },
        ];
        let forward_moves = slide(from, forward_dir, self.color, board, forward_dist, Cannot);
        let capture_moves: Vec<Vec2> = capture_dirs.iter().flat_map(
                                       |&dir| slide(from, dir, self.color, board, 1, Must)).collect();
        [&forward_moves[..], &capture_moves[..]].concat()
      }
      Knight => KNIGHT_DIRECTIONS.iter().flat_map(
                |&dir| slide(from, dir, self.color, board, 1, Can)).collect(),
      Bishop => DIAGONALS.iter().flat_map(
                |&dir| slide(from, dir, self.color, board, 8, Can)).collect(),
      Rook   => STRAIGHT_LINES.iter().flat_map(
                |&dir| slide(from, dir, self.color, board, 8, Can)).collect(),
      Queen  => EIGHT_DIRECTIONS.iter().flat_map(
                |&dir| slide(from, dir, self.color, board, 8, Can)).collect(),
      King   => EIGHT_DIRECTIONS.iter().flat_map(
                |&dir| slide(from, dir, self.color, board, 1, Can)).collect(),
    }
  }
}

pub fn display_moves(moves: &[Vec2]) {
  for y in 0..8 {
    for x in 0..8 {
      if moves.contains(&Sq { x: x as i32, y: y as i32 }) {
        print!("X");
      } else {
        print!(".")
      }
    }
    println!();
  }
}
