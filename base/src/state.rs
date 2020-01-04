use crate::special_rules::CastlingRights;
use crate::special_rules::KingState;
use crate::board::*;
use crate::color::{Color, *};
use crate::moves::*;

#[derive(Default)]
pub struct State {
  pub board: Board,
  pub active_color: Color,
  pub king_state: KingState,
  pub last_move: Option<Move>,
  pub castling_rights: CastlingRights,
  pub halfmove: u32,
  pub fullmove: u32,
}

impl State {
  pub fn new() -> State { State::default() }

  pub fn castle_moves(&self, color: Color) -> impl Iterator<Item = Move> + '_ {
    self.castling_rights.iter().filter(
      move |(_, clr, can_castle)| *clr == color && *can_castle
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
      .chain(self.en_passant())
  }
}
