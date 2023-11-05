use std::fmt::format;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const RESPONSE_TMPL: &'static str = "
HTTP/1.1 200 OK\r\n
Content-Type: text/html; charset=UTF-8\r\n
\r\n
";

const HTML: &'static str = "
<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
";

const HTML_404: &'static str = "
<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
";

pub fn listen(address: &str, port: i32) {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req = buf_reader
    .lines()
    .next()
    .unwrap()
    .unwrap();
    let (status_line, content) = match &req[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", HTML),
        "GET /sleep HTTP/1.1" => {
            println!("Sleeping...");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", HTML)
        },
        _ => ("HTTP/1.1 404 NOT FOUND", HTML_404),
    };
    let rsp = format!(
        "{status_line}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{html}", 
        html=content
    );
    println!("{}", rsp);
    stream.write_all(rsp.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Response: {}", rsp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
