use std::io::BufReader;
use crate::ui::UI;
use engine::engines::Engine;

pub enum Player {
  Human { ui: Box<dyn UI> },
  Computer { engine: Box<dyn Engine> },
  Uci { reader: BufReader }
}
