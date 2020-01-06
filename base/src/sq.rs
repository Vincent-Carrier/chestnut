use std::ops::Mul;
use std::ops::Add;

pub type SqSize = i8;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Vec2 {
  pub x: SqSize, pub y: SqSize
}

pub type Sq = Vec2;


impl Add<Vec2> for Vec2 {
  type Output = Vec2;
  fn add(self, rhs: Vec2) -> Vec2 {
    Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

impl Mul<SqSize> for Vec2 {
  type Output = Vec2;
  fn mul(self, rhs: SqSize) -> Vec2 {
    Vec2 { x: self.x * rhs, y: self.y * rhs }
  }
}

impl Vec2 {
  const fn rotate(self) -> Vec2 {
    Vec2 { x: -self.y, y: self.x }
  }

  pub fn inside_board(self) -> bool {
    0 <= self.x && self.x <= 7 &&
    0 <= self.y && self.y <= 7
  }


  pub fn parse(string: &str) -> Sq {
    let mut chars = string.bytes();
    let letter = chars.next().unwrap();
    let digit = chars.next().unwrap();
    Sq { x: (letter - 97) as SqSize, y: (56 - digit) as SqSize }
  }
}

pub struct RotationIter {
  dir: Vec2
}

impl Iterator for RotationIter {
  type Item = Vec2;

  fn next(&mut self) -> Option<Self::Item> {
    self.dir = self.dir.rotate();
    Some(self.dir)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (4, Some(4))
  }
}

pub fn four_directions(dir: Vec2) -> std::iter::Take<RotationIter> {
  RotationIter { dir }.take(4)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_a_sq() {
    assert_eq!(Sq::parse("a8"), Sq { x: 0, y: 0 });
    assert_eq!(Sq::parse("h1"), Sq { x: 7, y: 7 });
  }
}
