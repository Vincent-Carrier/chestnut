use crate::sq::Sq;
use crate::state::State;
use crate::piece::PieceKind::Pawn;
use crate::moves::Move;

impl State {
  pub fn en_passant(&self) -> Vec<Move> {
    if let Some(last_move) = self.last_move {
      if let Move::Normal { from, to, piece, .. } = last_move {
        if piece.kind == Pawn
          && from.y == self.active_color.opposite().pawn_row()
          && to.y == self.active_color.en_passant_row()
        {
          return [-1, 1]
            .iter()
            .map(|n| Sq { x: to.x, y: to.x + n })
            .filter(|&sq| {
              sq.inside_board()
                && self.board[sq]
                  .filter(|p| p.kind == Pawn && p.color == self.active_color)
                  .is_some()
            })
            .map(|sq| Move::EnPassant {
              from: sq,
              to: Sq { x: to.x, y: sq.y + self.active_color.forward() },
              capture: to
            })
            .collect();
        }
      }
    }
    return vec![];
  }
}
