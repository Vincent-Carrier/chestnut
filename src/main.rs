extern crate vampirc_uci;

mod user_move;
mod player;
mod ui;
mod game;
mod uci_engine;

use crate::game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}
