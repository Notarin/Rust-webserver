use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        exit(0)
    })
    .expect("Error setting Ctrl-C handler");
    let listener = TcpListener::bind("127.0.0.1:80")
    .unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer =  [0; 1024];
    stream.read(&mut buffer).unwrap();
    // #Debug Only# println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    let contents =
    fs::read_to_string(filename)
    .expect("test");
    let response =
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();    
}