#[macro_use]
extern crate lazy_static;

mod board;
mod color;
mod slide;
mod piece;
mod sq;
mod moves;
mod state;
mod player;
mod ui;
mod game;

use crate::game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}
