use crate::ui::UI;
use engine::engine::Engine;

pub enum Player {
  Human { ui: Box<dyn UI> },
  Computer { engine: Box<dyn Engine> }
}
