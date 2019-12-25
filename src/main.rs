mod board;
mod color;
mod movement;
mod piece;
mod sq;

use crate::board::*;
use crate::color::Color::*;
use crate::piece::*;
use crate::piece::PieceKind::*;
use crate::sq::*;

fn main() {
  let board = &Board::from_file("boards/initial.txt");
  let piece = Piece { kind: Bishop, color: White };
  let moves = piece.moves(Sq { x: 4, y: 4 }, board);
  println!("Moves: {:?}", moves);
}
