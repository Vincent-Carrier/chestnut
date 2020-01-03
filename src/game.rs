use core::color::Color;
use crate::player::Player;
use crate::player::PlayerKind::Human;
use crate::ui::CLI;
use core::color::Color::*;
use core::state::KingState::Checkmate;
use core::state::State;

pub struct Game {
  state: State,
  players: HashMap<Color, Player>
}

impl Game {
  pub fn new() -> Game {
    Game {
      state: State::new(),
      players: hashmap![
        White => Human { ui: CLI::new() },
        Black => Computer { engine: RandomEngine::new() }
      ]
    }
  }

  pub fn start(&mut self) {
    while self.state.king_state != Checkmate {
      let player = self.players[self.state.active_color];
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
