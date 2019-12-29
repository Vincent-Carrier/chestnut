use crate::state::KingState::Checkmate;
use crate::color::Color::*;
use crate::player::Player;
use crate::player::PlayerKind::Human;
use crate::state::State;
use crate::ui::CLI;

pub struct Game {
  state: State,
  white_player: Player,
  black_player: Player,
}

impl Game {
  pub fn new() -> Game {
    Game {
      state: State::new(),
      white_player: Player { kind: Human(CLI::new()) },
      black_player: Player { kind: Human(CLI::new()) },
    }
  }

  pub fn start(&mut self) {
    while self.state.king_state != Checkmate {
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
