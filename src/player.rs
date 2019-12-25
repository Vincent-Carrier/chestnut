use crate::color::*;
use crate::ui::UI;

pub enum PlayerKind {
  Human(Box<dyn UI>), Computer
}

pub struct Player {
  pub color: Color,
  pub kind: PlayerKind
}
