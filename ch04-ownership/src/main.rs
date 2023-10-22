pub fn main() {
  let s = String::from("hello");
  show(&s);
  let s2 = append(&s, String::from(", world"));
  println!("{}", s2);

}

fn show(s: &String) {
  println!("{s}")
}

fn append(s: &String, s2: String) -> String {
  let mut ret = String::from(s);
  ret.push_str(&s2);
  ret
}