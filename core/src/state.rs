use crate::sq::Sq;
use crate::piece::Piece;
use crate::color::{Color,*};
use crate::board::*;
use crate::moves::*;

pub type History = Vec<Move>;

#[derive(PartialEq)]
pub enum KingState {
  Safe, Check, Checkmate, Stalemate
}

pub use self::KingState::*;

#[derive(Clone, Copy, Default)]
struct CastlingRights {
  cannot_white_queen_castle: bool,
  cannot_white_king_castle: bool,
  cannot_black_queen_castle: bool,
  cannot_black_king_castle: bool,
}

pub struct State {
  pub board: Board,
  pub active_color: Color,
  pub king_state: KingState,
  pub last_move: Option<Move>,
  castling_rights: CastlingRights,
}

impl State {
  pub fn new() -> State {
    State {
      board: *INITIAL_BOARD,
      active_color: White,
      king_state: Safe,
      last_move: None,
      castling_rights: CastlingRights::default(),
    }
  }

  pub fn castle_rights(&self) -> Vec<Side> {
    let r = self.castling_rights;
    let mut result = vec![];
    match self.active_color {
      White => {
        if !r.cannot_white_queen_castle { result.push(Side::Queen) };
        if !r.cannot_white_king_castle { result.push(Side::King) };
      },
      Black => {
        if !r.cannot_black_queen_castle { result.push(Side::Queen) };
        if !r.cannot_black_king_castle { result.push(Side::King) };
      }
    }
    result
  }

  // pub fn reduce(&self, mv: Move) -> State {
  //   State {
  //     board: mv.execute(self.board.clone()),
  //     active_color: self.active_color.opposite(),
  //     king_state: Safe, // TODO
  //     last_move: Some(mv),
  //     castling_rights: self.castling_rights,
  //   }
  // }

  // Pseudo-legal because these might theoretically include moves
  // that would let the king be captured
  pub fn pseudo_legal_moves(&self) -> impl Iterator<Item = Move> + '_ {
    self.board.pieces_of(self.active_color).flat_map(|(from, piece)| {
      piece.moves(from, &self.board).map(|to|
        Move::Normal { from, to, piece, capture: self.board[to] }
      )
    })
  }
}

