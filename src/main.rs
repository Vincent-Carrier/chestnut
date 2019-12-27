#[macro_use]
extern crate lazy_static;
extern crate regex;

mod board;
mod color;
mod movement;
mod piece;
mod sq;
mod moves;
mod state;
mod player;
mod ui;
mod game;

use crate::game::Game;

fn main() {
  let game = Game::new();
  game.start();
}
