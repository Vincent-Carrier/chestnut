mod user_move;
mod cli;
mod game;

use uci::engine::UciEngine;
use crate::cli::CLI;
use engine::engine::Engine;
use base::player::Player;
use crate::game::Game;

fn parse_player_type(string: &str) -> Box<dyn Player> {
  match string {
    "cli" => Box::from(CLI {}),
    "engine" => Box::from(Engine::new()),
    "uci" => Box::from(UciEngine {}),
    _ => panic!("Unexecpted player type")
  }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let player1 = parse_player_type(&args[1]);
  let player2 = parse_player_type(&args[2]);
  let mut game = Game::new(player1, player2);
  game.start();
}
