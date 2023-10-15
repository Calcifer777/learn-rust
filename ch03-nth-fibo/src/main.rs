use std::io;

pub fn main() {
  println!("
    This script generates the nth Fibonacci number.
    Write a number:
  ");

  let n = get_num();

  let out = fibo(n);

  println!("The {n}th Fibonacci number is {out}!");
    
}

fn get_num() -> u32 {
  let mut s = String::new();
  io::stdin()
    .read_line(&mut s)
    .expect("Could not read number");

  match s.trim().parse() {
    Ok(n) => n ,
    Err(_) => panic!("Can't parse {s} as number!"),
  }

}

fn fibo(n: u32) -> u32 {
  match n {
    1 => 1,
    2 => 2,
    _ => fibo(n-1) + fibo(n-2)
  }
}