#[macro_use] extern crate lazy_static;

mod player;
mod ui;
mod game;

use crate::game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}
