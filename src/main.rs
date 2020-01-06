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
    unknown => panic!(format!("Unexecpted player type {}", unknown))
  }
}

fn main() {
  let mut args = std::env::args().skip(1);
  let player1 = parse_player_type(&args.next().unwrap_or("cli".to_string()));
  let player2 = parse_player_type(&args.next().unwrap_or("engine".to_string()));
  let mut game = Game::new(player1, player2);
  game.start();
}
