struct UserMove {
  from: Sq, to: Sq
}

impl UserMove {
  pub fn validate(&self, s: &State) -> Result<Move, &str> {
    let content = s.board[self.from];
    if let Some(p) = content {
      let valid = p.color == s.active_color && self.from.inside_board() && self.to.inside_board();
      let mv = match p.kind {
        King => if self.from == king_sq(s.active_color) {
          match self.to {
            castle_sq(s.active_color, Side::Queen) => Move::Castle { side: Side::Queen }
          }
        }
      }
      Ok(mv)
    } else {
      Err("no piece at given location")
    }
  }
}

fn castle_sq(color: Color, side: Side) -> Sq {
  Sq { x: if let side::Queen { 2 } else { 6 }, y: color.home_row() }
}

fn king_sq(color: Color) -> {
  Sq { x: 4, y: color.home_row() }
}
