use std::io::prelude::*;
use crate::state::State;
use crate::moves::Move;

trait UI {
  fn prompt_move(&self, s: &State) -> dyn Move;
}

struct CLI {
  reader: Read, writer: Write
}

impl CLI {
  fn new() -> CLI {
    CLI { reader: stdin(), writer: stdout() }
  }
}

impl UI for CLI {
  fn prompt_move(&self, s: &State) -> dyn Move {
    writer.
  }
}

static SQ_REGEX: &str = r"[a-h][1-8]";
static PIECE_REGEX: &str = r"[BNRQK]";

