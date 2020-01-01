use crate::color::{Color,*};
use crate::board::*;
use crate::moves::*;

pub type History = Vec<Move>;

#[derive(PartialEq)]
pub enum KingState {
  Safe, Check, Checkmate, Stalemate
}

pub use self::KingState::*;

#[derive(Clone, Copy)]
struct CastlingRights {
  white_queen_castle: bool,
  white_king_castle: bool,
  black_queen_castle: bool,
  black_king_castle: bool,
}

pub struct State {
  pub board: Board,
  pub active_color: Color,
  pub king_state: KingState,
  pub last_move: Option<Move>,
  castling_rights: CastlingRights
}

impl State {
  pub fn new() -> State {
    State {
      board: INITIAL_BOARD.clone(),
      active_color: White,
      king_state: Safe,
      last_move: None,
      castling_rights: CastlingRights {
        white_queen_castle: true,
        white_king_castle: true,
        black_queen_castle: true,
        black_king_castle: true,
      }
    }
  }

  pub fn can_castle(&self, color: Color, side: Side) -> bool {
    let rights = &self.castling_rights;
    match (color, side) {
      (White, Side::Queen) => rights.white_queen_castle,
      (Black, Side::Queen) => rights.black_queen_castle,
      (Black, Side::King)  => rights.black_king_castle,
      (White, Side::King)  => rights.white_king_castle,
    }
  }

  pub fn reduce(&self, mv: Move) -> State {
    State {
      board: mv.execute(self),
      active_color: self.active_color.opposite(),
      king_state: Safe, // TODO
      last_move: Some(mv),
      castling_rights: self.castling_rights,
    }
  }

  pub fn all_moves(&self) -> Vec<Move> {
    let basic_moves = self.board.pieces_of(self.active_color).iter().map(
      |(sq, piece)| piece.moves(*sq, &self.board)
    ).collect();
    // TODO: Castling, EnPassant, Promotion
  }
}
