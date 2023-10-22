use std::cmp::Reverse;


pub fn main() {
  let r = Rectangle {w: 0.1, h:0.3};
  println!("The area of the rectangle {:?} is {:.2}", r, r.area());

  let s = Rectangle::make_square(1.5);
  println!("The area of the square {:?} is {}", s, s.area());
}

#[derive(Debug)]
struct Rectangle {
  w: f32,
  h: f32
}

impl Rectangle {
  fn area(&self) -> f32 {
    self.w * self.h
  }

  fn widen(&mut self, x: f32) {
    self.w *= 1. + x;
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.w > other.w && self.h > other.h
  }

  fn make_square(size: f32) -> Rectangle {
    Rectangle {w: size, h: size}
  }
}