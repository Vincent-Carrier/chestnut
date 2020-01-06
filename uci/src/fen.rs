use crate::bridge::from_sq;
use base::piece::Piece;
use base::piece::PieceKind;
use base::moves::Side;
use base::piece::PieceKind::King;
use base::piece::PieceKind::Queen;
use base::prelude::*;
use vampirc_uci::UciFen;

pub fn uci_fen(state: &State) -> UciFen {
  UciFen::from(fen_string(state).as_str())
}

fn digit_to_char(digit: u32) -> char {
  std::char::from_digit(digit, 10).unwrap()
}

fn fen_board_string(board: &Board) -> String {
  let mut string = String::with_capacity(96);
  let mut empty_sq_count = 0;

  for (sq, content) in board.iter() {
    if let Some(piece) = content {
      if empty_sq_count > 0 {
        string.push(digit_to_char(empty_sq_count));
        empty_sq_count = 0;
      };
      string.push(piece.char());
    } else {
      empty_sq_count += 1;
    }

    if sq.x == 7 {
      if empty_sq_count > 0 {
        string.push(digit_to_char(empty_sq_count));
      };
      string.push('/');
      empty_sq_count = 0;
    }
  }

  string.pop(); // Remove extraneous slash
  string
}

fn kind(side: Side) -> PieceKind {
  match side {
    Side::Queen => Queen,
    Side::King => King,
  }
}

pub fn fen_string(state: &State) -> String {
  let mut string = fen_board_string(&state.board);
  string.push(' ');
  string.push(state.active_color.char());
  string.push(' ');

  let rights = &state.castling_rights;

  // ITERATE THROUGH THE MAP TWICE!! LIVE LIKE THERE'S NO TOMORROW!!
  if rights.iter().count() == 0 {
    string.push('-');
  } else {
    for (&(color, side), &has_right) in rights.iter() {
      if has_right {
        let piece = Piece { kind: kind(side), color };
        string.push(piece.char());
      }
    }
  };

  if let Some(mv) = state.last_move {
    if let Move::Normal { from, .. } = mv {
      let y = from.y + state.active_color.opposite().forward();
      let sq = from_sq(Sq { x: from.x, y });
      string.push_str(&format!(" {}", sq));
    }
  } else {
    string.push_str(" -");
  }

  for i in &[state.halfmoves, state.fullmoves] {
    string.push(' ');
    string.push(digit_to_char(*i));
  }

  string
}

#[cfg(test)]
mod tests {
  use base::prelude::*;
  use super::*;

  #[test]
  fn fen_string_initial_state() {
    assert_eq!(
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      fen_string(&State::new())
    );
    assert_eq!(
      "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
      fen_string(&State::with("e2e4 c7c5"))
    )
  }
}
