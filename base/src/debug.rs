
pub fn display_moves(moves: &[Vec2]) {
  for y in 0..8 {
    for x in 0..8 {
      if moves.contains(&Sq { x: x as SqSize, y: y as SqSize }) {
        print!("X");
      } else {
        print!(".")
      }
    }
    println!();
  }
}
