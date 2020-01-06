use crate::uci_bridge::parse_move;
use crate::color::Color::White;
use crate::board::*;
use crate::color::Color;
use crate::moves::*;
use crate::special_rules::CastlingRights;
use crate::special_rules::KingState;
use crate::special_rules::KingState::Safe;

#[derive(Default)]
pub struct State {
  pub board: Board,
  pub active_color: Color,
  pub king_state: KingState,
  pub last_move: Option<Move>,
  pub castling_rights: CastlingRights,
  pub halfmoves: u32, // since last capture or pawn advance
  pub fullmoves: u32,
}

impl State {
  pub fn new() -> State {
    State { fullmoves: 1, ..State::default() }
  }

  pub fn with(string: &str) -> State {
    let mut state = State::new();
    let moves: Vec<Move> = string.split_ascii_whitespace().map(
      |move_str| parse_move(move_str.to_string(), &state.board)
    ).collect();
    for mv in moves {
      state.execute(mv);
    }
    state
  }

  pub fn castle_moves(&self, color: Color) -> impl Iterator<Item = Move> + '_ {
    self
      .castling_rights
      .iter()
      .filter(move |(&(clr, ..), &has_right)| clr == color && has_right)
      .map(|(&(.., side), ..)| Move::Castle { side })
  }

  pub fn execute(&mut self, mv: Move) {
    mv.execute(&mut self.board, self.active_color);
    self.active_color = self.active_color.opposite();
    self.king_state = Safe; // TODO;
    self.last_move = Some(mv);
    self.castling_rights = Default::default(); // TODO
    if self.active_color == White { self.fullmoves += 1 }
  }

  // "Pseudo"-legal because these might theoretically include moves
  // that would let the king be captured
  pub fn pseudo_legal_moves(&self) -> impl Iterator<Item = Move> + '_ {
    self
      .board
      .moves_of(self.active_color)
      .chain(self.castle_moves(self.active_color))
      .chain(self.en_passant())
  }
}
