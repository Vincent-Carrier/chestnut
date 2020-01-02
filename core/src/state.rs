use crate::en_passant::*;
use crate::board::*;
use crate::color::{Color, *};
use crate::moves::*;
use crate::piece::Piece;
use crate::piece::PieceKind::Pawn;
use crate::sq::Sq;

pub type History = Vec<Move>;

#[derive(PartialEq)]
pub enum KingState {
  Safe,
  Check,
  Checkmate,
  Stalemate,
}

pub use self::KingState::*;

pub type CastlingRights = [(Side, Color, bool); 4];

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
      board: INITIAL_BOARD.clone(),
      active_color: White,
      king_state: Safe,
      last_move: None,
      castling_rights: [
      (Side::Queen, White, true),
      (Side::King, White, true),
      (Side::Queen, Black, true),
      (Side::King, Black, true),
      ],
    }
  }

  pub fn castle_moves(&self, color: Color) -> impl Iterator<Item = Move> + '_ {
    self.castling_rights.iter().filter(
      |(side, clr, can_castle)| *clr == color && *can_castle
      ).map(|(side, ..)| Move::Castle { side: *side })
  }


  pub fn execute(&mut self, mv: Move) {
    mv.execute(&mut self.board, self.active_color);
    self.active_color = self.active_color.opposite();
    self.king_state = Safe; // TODO;
    self.last_move = Some(mv);
    self.castling_rights = self.castling_rights; // TODO
  }

  // "Pseudo"-legal because these might theoretically include moves
  // that would let the king be captured
  pub fn pseudo_legal_moves(&self) -> impl Iterator<Item = Move> + '_ {
    self.board.moves_of(self.active_color)
      .chain(self.castle_moves(self.active_color))
      .chain(self.en_passant().iter())
  }

  pub fn legal_moves(&self) -> impl Iterator<Item = Move> + '_ {
    self.pseudo_legal_moves().filter(|mv| !mv.self_check(self))
  }
}
