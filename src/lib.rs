
pub enum Shape {
  Rectangle(f32, f32),
  Square(f32),
  Triangle(f32, f32, f32),
}

impl Shape {
  pub fn area(&self) -> f32 {
    match self {
      Shape::Rectangle(_, _) => 1.0,
      Shape::Square(_) => 1.0,
      Shape::Triangle(_, _, _) => 1.0,
    }
  }
}