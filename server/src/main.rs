use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    println!("====================");

    let root = b"GET / HTTP/1.1\r\n";
    let style = b"GET /styles.css HTTP/1.1\r\n";
    let script = b"GET /bundle.js HTTP/1.1\r\n";
    let favicon = b"GET /favicon.ico HTTP/1.1\r\n";

    let response =
        if buffer.starts_with(root) {
            generate_response("index.html")
        } else if buffer.starts_with(style) {
            generate_response("styles.css")
        } else if buffer.starts_with(script) {
            generate_response("bundle.js")
        } else if buffer.starts_with(favicon) {
            generate_response("")
        } else {
            generate_404_response()
        };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn generate_response(request_source: &str) -> String {
    let mut response = String::new();

    response.push_str("HTTP/1.1");
    response.push_str(" 200");
    response.push_str(" OK\r\n\r\n");
    
    if !request_source.is_empty() {
        let path = format!("../client/public/{}", request_source);
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        response.push_str(&contents);
    }

    return response;
}

fn generate_404_response() -> String {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let path = format!("../client/public/404.html");

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    return response;
}