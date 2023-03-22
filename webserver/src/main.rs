use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Shutdown::Read;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
        handle_stream(stream);
    }

    println!("Hello, world!");
}

fn handle_stream(mut stream : TcpStream) {
    let reader = BufReader::new(&mut stream);

    let request_line = reader.lines().next().unwrap().unwrap();
    println!("{request_line}");
    let request_kind = request_line.split(" ").next().unwrap();

    println!("{request_line} -> {request_kind}");

    let (status, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let body = fs::read_to_string(filename).unwrap();
    let length = body.len();
    let headers = format!("Content-Length: {length}");

    let response = format!("{status}\r\n{headers}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
}
