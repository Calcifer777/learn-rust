
pub fn main() {
  println!("Hello, world");

  let home = IpAddr::V4(String::from("127.0.0.1"));

  println!("My home ip address is {home:?}");

  let i = 7;
  let x = Some(3);
  let y: Option<i32> = None;
  println!("Result: {:?}", y.map(|j| i+j));
}


#[derive(Debug)]
enum IpAddr {
  V4(String),
  V6(String)
}