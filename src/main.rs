mod board;
mod color;
mod movement;
mod piece;
mod sq;
mod moves;
mod state;
mod player;
mod ui;

use crate::board::*;
use crate::color::Color::*;
use crate::piece::*;
use crate::piece::PieceKind::*;
use crate::sq::*;

fn main() {
  let board = &Board::initial();
  let piece = Piece { kind: Bishop, color: White };
  let moves = piece.moves(Sq { x: 4, y: 4 }, board);
  println!("Moves:");
  movement::display_moves(&moves);
}
