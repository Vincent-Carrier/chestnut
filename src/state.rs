use crate::color::Color;
use crate::board::*;
use crate::moves::*;

pub type History = Vec<Move>;

pub enum KingState {
  Safe, Check, Checkmate, Stalemate
}

pub use self::KingState::*;

pub struct CastlingState {
  king_moved: bool,
  king_rook_moved: bool,
  queen_rook_moved: bool,
}

impl CastlingState {
  pub fn new() -> CastlingState {
    CastlingState {
      king_moved: true,
      king_rook_moved: true,
      queen_rook_moved: true,
    }
  }
}

pub struct State {
  pub board: Board,
  pub active_color: Color,
  pub king_state: KingState,
  pub white_castling_state: CastlingState,
  pub black_castling_state: CastlingState,
  pub last_move: Option<Move>,
}

impl State {
  pub fn reduce(&self, mv: Move) -> State {
    State {
      board: mv.execute(self),
      active_color: self.active_color.opposite(),
      king_state: Safe, // TODO
      white_castling_state: CastlingState::new(),
      black_castling_state: CastlingState::new(),
      last_move: None,
    }
  }
}
