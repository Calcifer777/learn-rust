use std::{fs::File, io::Write, path::Path};


fn main() {
    let path = Path::new("file.txt");
    let result = {
        File::create(path)
        .map(|mut fp| {fp.write_all("hi".as_bytes())})
    };
    match result {
        Ok(_) => print!("Written to file!"),
        Err(e) => println!("Could not write to file; Error: {}", e),
    };

}

fn make_error() {
  panic!("Danger!")
}