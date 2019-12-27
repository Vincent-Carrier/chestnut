use crate::player::Player;
use crate::color::Color::Black;
use crate::color::Color::White;
use crate::player::PlayerKind::Human;
use crate::board::Board;
use crate::state::State;
use crate::ui::CLI;

pub struct Game {
  state: State
}

impl Game {
  pub fn new() -> Game {
    Game {
      state: State {
        board: Board::from_file("boards/initial.txt"),
        history: vec![],
        active_player: Player { color: White, kind: Human(CLI::new()) },
        passive_player: Player { color: Black, kind: Human(CLI::new()) },
        in_check: false
      }
    }
  }

  pub fn start(&self) {
    loop {
      match self.active_player.kind {
        Human(cli) => {

          let mv = cli.prompt_move()
        },
        _ => panic!("Not implemented")
      }
    }
  }
}
