mod user_move;
mod cli;
mod game;

use crate::game::GameBuilder;


fn main() {
  let mut args = std::env::args().skip(1);
  let mut builder = GameBuilder::new();
  if let Some(player1) = args.next() { builder.player1(player1); }
  if let Some(player2) = args.next() { builder.player2(player2); }
  if let Some(_) = args.next() { builder.observe(true); }
  let mut game = builder.build();
  game.start();
}
