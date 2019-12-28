use crate::color::Color::*;
use crate::state::KingState::Safe;
use crate::player::Player;
use crate::player::PlayerKind::Human;
use crate::board::Board;
use crate::state::{State, CastlingState};
use crate::ui::CLI;

pub struct Game {
  state: State,
  white_player: Player,
  black_player: Player,
}

impl Game {
  pub fn new() -> Game {
    Game {
      state: State {
        board: Board::from_file("boards/initial.txt"),
        king_state: Safe,
        active_color: White,
        white_castling_state: CastlingState::new(),
        black_castling_state: CastlingState::new(),
        last_move: None,
      },
      white_player: Player { kind: Human(CLI::new()) },
      black_player: Player { kind: Human(CLI::new()) },
    }
  }

  pub fn start(&mut self) {
    loop {
      let player = match self.state.active_color {
        White => &self.white_player,
        Black => &self.black_player,
      };
      let mv = match &player.kind {
        Human(cli) => {
          cli.prompt_turn(&self.state);
          loop {
            if let Some(m) = cli.prompt_move(&self.state) { break m } else { continue }
          }
        },
        _ => panic!("Not implemented")
      };
      let new_state = self.state.reduce(mv);
      self.state = new_state;
    }
  }
}
