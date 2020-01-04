use crate::color::Color;
use crate::piece::PieceKind::{Pawn, Queen, King};
use crate::piece::Piece;
use crate::sq::*;
use crate::board::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Side { Queen, King }

impl Side {
  fn king_destination_sq(self, color: Color) -> Sq {
    let x = if let Side::Queen = self { 2 } else { 6 };
    Sq { x, y: color.home_row() }
  }

  fn rook_destination_sq(self, color: Color) -> Sq {
    let x = if let Side::Queen = self { 3 } else { 5 };
    Sq { x, y: color.home_row() }
  }

  fn original_rook_sq(self, color: Color) -> Sq {
    let x = if let Side::Queen = self { 0 } else { 7 };
    Sq { x, y: color.home_row() }
  }

  pub fn iter() -> std::slice::Iter<'static, Side> {
    static SIDES: [Side; 2] = [Side::Queen, Side::King];
    SIDES.iter()
  }
}


#[derive(Clone, Copy)]
pub enum Move {
  Normal { piece: Piece, from: Sq, to: Sq, capture: Option<Piece> },
  Castle { side: Side },
  EnPassant { from: Sq, to: Sq, capture: Sq },
}

pub use self::Move::*;

impl Move {
  // This is only for user moves.For computer-generated moves,
  // king safety will be accounted for by the eval function
  pub fn self_check(self, board: &mut Board, color: Color) -> bool {
    self.execute(board, color);
    let king_sq = board.pieces_of(color).find(|(_, p)| p.kind == King).unwrap().0;
    let is_check = board.is_threatened(king_sq, color.opposite());
    self.undo(board, color);
    is_check
  }

  pub fn execute(self, board: &mut Board, color: Color) {
    match self {
      Normal { piece, from, to, .. } => {
        if piece.kind == Pawn && to.y == color.opposite().home_row() {
          board[to] = Some(Piece { kind: Queen, color });
        } else {
          board[to] = Some(piece);
        }
        board[from] = None;
      },
      EnPassant { from, to, capture } => {
        board[to] = Some(Piece { kind: Pawn, color });
        board[capture] = None;
        board[from] = None;
      },
      Castle { side } => {
        let original_king_sq = Sq { x: 4, y: color.home_row() };
        let original_rook_sq = side.original_rook_sq(color);
        let king_sq = side.king_destination_sq(color);
        let rook_sq = side.rook_destination_sq(color);
        board[king_sq] = board[original_king_sq];
        board[rook_sq] = board[original_rook_sq];
        board[original_king_sq] = None;
        board[original_rook_sq] = None;
      },
    }
  }

  pub fn undo(self, board: &mut Board, color: Color) {
    match self {
      Normal { piece, from, to, capture } => {
        board[to] = capture;
        board[from] = Some(piece);
      },
      Castle { side } => {
        let original_king_sq = Sq { x: 4, y: color.home_row() };
        let original_rook_sq = side.original_rook_sq(color);
        let king_sq = side.king_destination_sq(color);
        let rook_sq = side.rook_destination_sq(color);
        board[original_king_sq] = board[king_sq];
        board[original_rook_sq] = board[rook_sq];
        board[king_sq] = None;
        board[rook_sq] = None;
      },
      EnPassant { from, to, capture } => {
        board[capture] = Some(Piece { kind: Pawn, color: color.opposite() });
        board[from] = Some(Piece { kind: Pawn, color });
        board[to] = None;
      }
    }
  }
}

