#[macro_use] extern crate lazy_static;

mod user_move;
mod player;
mod ui;
mod game;

use crate::game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}
