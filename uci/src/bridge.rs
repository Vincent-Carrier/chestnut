use base::piece::PieceKind;
use base::prelude::*;
use base::sq::SqSize;
use vampirc_uci::*;

pub fn from_sq(sq: Sq) -> UciSquare {
  UciSquare { file: (sq.x as u8 + 97).into(), rank: 8 - sq.y as u8 }
}

pub fn into_sq(sq: UciSquare) -> Sq {
  Sq { x: (sq.file  as u8 - 97) as SqSize, y: 8 - sq.rank as SqSize }
}

pub fn into_move(mv: Move) -> UciMove {
  match mv {
    // TODO: special cases
    Move::Normal { from, to, .. } => {
      UciMove { from: from_sq(from), to: from_sq(to), promotion: None }
    }
    _ => panic!("Can't convert move to UCI"),
  }
}

pub fn from_move(uci_move: UciMove, board: &Board) -> Move {
  unimplemented!()
}

// TODO: Macro this sh*t out
fn from_piece(piece: PieceKind) -> UciPiece {
  match piece {
    PieceKind::Pawn => UciPiece::Pawn,
    PieceKind::Knight => UciPiece::Knight,
    PieceKind::Bishop => UciPiece::Bishop,
    PieceKind::Rook => UciPiece::Rook,
    PieceKind::Queen => UciPiece::Queen,
    PieceKind::King => UciPiece::King,
  }
}

fn into_piece_kind(piece: UciPiece) -> PieceKind {
  match piece {
    UciPiece::Pawn => PieceKind::Pawn,
    UciPiece::Knight => PieceKind::Knight,
    UciPiece::Bishop => PieceKind::Bishop,
    UciPiece::Rook => PieceKind::Rook,
    UciPiece::Queen => PieceKind::Queen,
    UciPiece::King => PieceKind::King,
  }
}
