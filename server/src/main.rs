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

    let request = String::from_utf8_lossy(&buffer[..]);
    let mut response = String::new();

    if !request.is_empty() {
        let v: Vec<&str> = request.trim().split(' ').collect();
        if v.len() > 2 {
            println!("Request method: {}; path: {}", v[0], v[1]);
            println!("====================");

            if !v[0].is_empty() && v[0].len() == 3 && v[0].starts_with("GET") {
                response = get_response(&v[1]);
            }
        }
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_response(request: &str) -> String {

    let (status_line, path) =
        if request.len() == 1 && request.starts_with("/") {
            (get_status(200), get_source_path("index.html"))
        } else if request.starts_with("/styles.css") {
            (get_status(200), get_source_path("styles.css"))
        } else if request.starts_with("/bundle.js") {
            (get_status(200), get_source_path("bundle.js"))
        } else if request.starts_with("/favicon.ico") {
            (get_status(200), get_source_path(""))
        } else {
            (get_status(404), get_source_path("404.html"))
        };

    let mut response = String::new();
    response.push_str(&status_line);

    if !path.is_empty() {
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        response.push_str(&contents);
    }

    return response;
}

fn get_source_path(source: &str) -> String {

    let mut path = String::new();

    if !source.is_empty() {
        path = format!("../client/public/{}", &source);
    }

    return path;
}

fn get_status(status: u16) -> String {

    let status_line: String;
    if status == 200 {
        status_line = String::from("HTTP/1.1 200 OK\r\n\r\n");
    } else {
        status_line = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
    }

    return status_line;
}