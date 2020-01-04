use crate::board::Board;
use crate::moves::Side;
use crate::moves::*;
use crate::piece::Piece;
use crate::piece::PieceKind;
use crate::piece::PieceKind::King;
use crate::piece::PieceKind::Queen;
use crate::sq::Sq;
use crate::state::State;
use vampirc_uci::UciFen;
use vampirc_uci::UciSquare;

impl Into<UciFen> for State {
  fn into(self) -> UciFen {
    UciFen::from(self.fen_string().as_str())
  }
}

impl Into<State> for UciFen {
  fn into(self) -> State {
    unimplemented!()
  }
}

fn digit_to_char(digit: u32) -> char {
  std::char::from_digit(digit, 10).unwrap()
}

impl Board {
  fn fen_string(&self) -> String {
    let mut string = String::with_capacity(96);
    let mut empty_sq_count = 0;

    for (i, (_, content)) in self.iter().enumerate() {
      if let Some(piece) = content {
        if empty_sq_count > 0 {
          string.push(digit_to_char(empty_sq_count));
          empty_sq_count = 0;
        };
        string.push(piece.char());
      } else {
        empty_sq_count += 1;
      }
      if (i + 1) % 8 == 0 {
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
}

impl From<Side> for PieceKind {
  fn from(side: Side) -> PieceKind {
    match side {
      Side::Queen => Queen,
      Side::King => King,
    }
  }
}

impl State {
  pub fn fen_string(&self) -> String {
    let mut string = self.board.fen_string();
    string.push(' ');
    string.push(self.active_color.char());
    string.push(' ');

    let rights = &self.castling_rights;

    // ITERATE THROUGH THE MAP TWICE!! LIVE LIKE THERE'S NO TOMORROW!!
    if rights.iter().count() == 0 {
      string.push('-');
    } else {
      for (&(color, side), &has_right) in rights.iter() {
        if has_right {
          let piece = Piece { kind: side.into(), color };
          string.push(piece.char());
        }
      }
    };

    if let Some(mv) = self.en_passant().get(0) {
      if let Move::Normal { from, to, .. } = mv {
        let sq: UciSquare = Sq { x: to.x, y: from.y }.into();
        string.push_str(&format!("{}", sq));
      }
    } else {
      string.push_str(" -");
    }

    for i in &[self.halfmoves, self.fullmoves] {
      string.push(' ');
      string.push(digit_to_char(*i));
    }

    string
  }
}

#[cfg(test)]
mod tests {
  use crate::state::State;

  #[test]
  fn fen_string_initial_state() {
    println!("expected: {}", State::new().fen_string());
    assert_eq!(
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      State::new().fen_string()
    )
  }
}
