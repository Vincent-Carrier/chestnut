use std::collections::HashMap;
use crate::sq::*;
use crate::piece::*;
use std::fs::File;

pub struct Board {
  pieces: HashMap<Sq, Piece>
}

impl Board {
  pub fn at(&self, sq: Sq) -> Option<&Piece> {
    self.pieces.get(&sq)
  }

  pub fn new() -> Board {
    Board { pieces: HashMap::new() }
  }

  pub fn from(file: String) {
    let f = File::open(file).expect("File not found");

  }
}
