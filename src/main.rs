
mod utils;

pub fn main() {
  println!("Hello, world");
  let _ = learn_rust::Shape::Rectangle(2.0, 3.0);
  utils::greet::greet();
}