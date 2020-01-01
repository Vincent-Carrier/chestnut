pub type SqSize = i8;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Vec2 {
  pub x: SqSize, pub y: SqSize
}

pub type Sq = Vec2;
