#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate strum_macros;
extern crate strum;

pub mod board;
pub mod board_iter;
pub mod color;
pub mod piece;
pub mod sq;
pub mod moves;
pub mod state;
pub mod uci_bridge;
pub mod prelude;
mod slide;
mod en_passant;
