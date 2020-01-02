use crate::piece::PieceKind::Pawn;
use crate::color::Color;
use crate::piece::PieceKind::King;
use crate::piece::Piece;
use crate::sq::*;
use crate::board::*;
use crate::state::*;

#[derive(Clone, Copy)]
pub enum Side { Queen, King }

impl Side {
  fn king_destination_sq(self, color: Color) -> Sq {
    let x = if let _ = Side::Queen { 2 } else { 6 };
    Sq { x, y: color.home_row() }
  }

  fn rook_destination_sq(self, color: Color) -> Sq {
    let x = if let _ = Side::Queen { 3 } else { 5 };
    Sq { x, y: color.home_row() }
  }

  fn original_rook_sq(self, color: Color) -> Sq {
    let x = if let _ = Side::Queen { 0 } else { 7 };
    Sq { x, y: color.home_row() }
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
  // king safety will be accounted for by eval function
  pub fn valid(self, s: &State) -> bool {
    let special_rules = match self {
      Castle { side } => {
        true // TODO
      },
      EnPassant { from, to, capture } => {
        true
      },
      _ => true,
    };
    self.execute(&mut s.board, s.active_color);
    let king_sq = s.board.pieces_of(s.active_color).find(|(sq, p)| p.kind == King).unwrap().0;
    let is_check = s.board.is_threatened(king_sq, s.active_color.opposite());
    self.undo(&mut s.board, s.active_color);
    is_check && special_rules
  }

  pub fn execute(self, board: &mut Board, color: Color) {
    match self {
      Normal { piece, from, to, .. } => {
        board[to] = Some(piece);
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

