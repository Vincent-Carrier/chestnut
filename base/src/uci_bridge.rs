use crate::sq::SqSize;
use crate::piece::PieceKind;
use vampirc_uci::*;
use crate::prelude::*;

impl Into<UciSquare> for Sq {
  fn into(self) -> UciSquare {
    UciSquare {
      file: (self.x as u8 + 97).into(),
      rank: 8 - self.y as u8
    }
  }
}

impl Into<Sq> for UciSquare {
  fn into(self) -> Sq {
    Sq {
      x: (97 - self.file as u8) as SqSize,
      y: 8 + self.rank as SqSize
    }
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
