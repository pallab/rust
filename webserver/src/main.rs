use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Shutdown::Read;
use std::time::Duration;
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming().take(20) {
        let stream = stream.unwrap();

        println!("New Connection");
        thread_pool.execute(|| handle_stream(stream));
    }

    println!("Shutting Down");
}

fn handle_stream(mut stream : TcpStream) {
    let reader = BufReader::new(&mut stream);

    let request_line = reader.lines().next().unwrap().unwrap();
    println!("{request_line}");
    let request_kind = request_line.split(" ").next().unwrap();

    println!("{request_line} -> {request_kind}");

    let (status, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /slow HTTP/1.1" => {
            std::thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let body = fs::read_to_string(filename).unwrap();
    let length = body.len();
    let headers = format!("Content-Length: {length}");

    let response = format!("{status}\r\n{headers}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
}
