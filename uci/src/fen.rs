use base::piece::PieceKind::King;
use base::piece::PieceKind::Queen;
use base::prelude::*;
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

    for (sq, content) in self.iter() {
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

    if let Some(mv) = self.last_move {
      if let Move::Normal { from, .. } = mv {
        let y = from.y + self.active_color.opposite().forward();
        let sq: UciSquare = Sq { x: from.x, y }.into();
        string.push_str(&format!(" {}", sq));
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
  use base::prelude::*;

  #[test]
  fn fen_string_initial_state() {
    assert_eq!(
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      State::new().fen_string()
    );
    assert_eq!(
      "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
      State::with("e2e4 c7c5").fen_string()
    )
  }
}
