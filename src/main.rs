mod user_move;
mod cli;
mod game;

use crate::game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}
