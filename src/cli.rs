use base::player::Player;
use std::io::stdin;
use base::prelude::*;

impl Player for CLI {
  fn prompt_turn(&self, state: &State) {
    println!("{}", state.board);
    println!("It's your turn to play!");
    println!("Please enter your move: ");
  }

  fn post_move(&self, state: &State) -> Move {
    let mut move_str = String::new();
    stdin().read_line(&mut move_str).unwrap();
    Move::parse(&move_str, &state.board)
  }
}

#[derive(Default)]
pub struct CLI {}
