pub type SqSize = i8;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Vec2 {
  pub x: SqSize, pub y: SqSize
}

pub type Sq = Vec2;


impl std::ops::Add<Vec2> for Vec2 {
  type Output = Vec2;
  fn add(self, rhs: Vec2) -> Vec2 {
    Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

impl std::ops::Mul<SqSize> for Vec2 {
  type Output = Vec2;
  fn mul(self, rhs: SqSize) -> Vec2 {
    Vec2 { x: self.x * rhs, y: self.y * rhs }
  }
}

impl Vec2 {
  fn scale(self, n: SqSize) -> Vec2 {
    Vec2 { x: self.x * n, y: self.y * n }
  }

  const fn rotate(self) -> Vec2 {
    Vec2 { x: -self.y, y: self.x }
  }

  pub fn inside_board(self) -> bool {
    0 <= self.x && self.x <= 7 &&
    0 <= self.y && self.y <= 7
  }
}

struct RotationIter {
  dir: Vec2
}

impl Iterator for RotationIter {
  type Item = Vec2;

  fn next(&mut self) -> Option<Self::Item> {
    self.dir = self.dir.rotate();
    Some(self.dir)
  }
}

pub fn four_directions(dir: Vec2) -> std::iter::Take<RotationIter> {
  RotationIter { dir }.take(4)
}
