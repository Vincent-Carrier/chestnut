use crate::piece::PieceKind;
use crate::prelude::*;
use crate::sq::SqSize;
use vampirc_uci::*;

impl Into<UciSquare> for Sq {
  fn into(self) -> UciSquare {
    UciSquare { file: (self.x as u8 + 97).into(), rank: 8 - self.y as u8 }
  }
}

impl Into<Sq> for UciSquare {
  fn into(self) -> Sq {
    Sq { x: (self.file  as u8 - 97) as SqSize, y: 8 - self.rank as SqSize }
  }
}

pub fn parse_sq(string: &str) -> UciSquare {
  let mut chars = string.chars();
  UciSquare {
    file: chars.next().unwrap(),
    rank: chars.next().unwrap().to_digit(10).unwrap() as u8,
  }
}

pub fn parse_move(s: String, board: &Board) -> Move {
  let from = parse_sq(&s[0..2]).into();
  let to = parse_sq(&s[2..]).into();
  Move::Normal {
    from, to, piece: board[from].unwrap(), capture: board[to],
  }
}

impl Into<UciMove> for Move {
  fn into(self) -> UciMove {
    match self {
      // TODO: special cases
      Move::Normal { from, to, .. } => {
        UciMove { from: from.into(), to: to.into(), promotion: None }
      }
      _ => panic!("Can't convert move to UCI"),
    }
  }
}

// TODO: Macro this sh*t out
impl Into<UciPiece> for PieceKind {
  fn into(self) -> UciPiece {
    match self {
      PieceKind::Pawn => UciPiece::Pawn,
      PieceKind::Knight => UciPiece::Knight,
      PieceKind::Bishop => UciPiece::Bishop,
      PieceKind::Rook => UciPiece::Rook,
      PieceKind::Queen => UciPiece::Queen,
      PieceKind::King => UciPiece::King,
    }
  }
}

impl Into<PieceKind> for UciPiece {
  fn into(self) -> PieceKind {
    match self {
      UciPiece::Pawn => PieceKind::Pawn,
      UciPiece::Knight => PieceKind::Knight,
      UciPiece::Bishop => PieceKind::Bishop,
      UciPiece::Rook => PieceKind::Rook,
      UciPiece::Queen => PieceKind::Queen,
      UciPiece::King => PieceKind::King,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::*;
  use vampirc_uci::UciSquare;

  #[test]
  fn parse_a_sq() {
    assert_eq!(UciSquare::from('e', 4), parse_sq("e4"));
    assert_eq!(UciSquare::from('a', 1), parse_sq("a1"));
    assert_eq!(UciSquare::from('h', 8), parse_sq("h8"));
  }

  #[test]
  fn convert_uci_to_a8() {
    let sq: Sq = UciSquare::from('a', 8).into();
    assert_eq!(sq, Sq { x: 0, y: 0 });
  }

  #[test]
  fn convert_uci_to_h1() {
    let sq: Sq = UciSquare::from('h', 1).into();
    assert_eq!(sq, Sq { x: 7, y: 7 });
  }
}
