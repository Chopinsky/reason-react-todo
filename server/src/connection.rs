#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use thread_utils::ThreadPool;

enum REST {
    NONE,
    GET,
    POST,
    PUT,
    DELETE,
}

struct Request {
    rest_method: REST,
    request_path: String,
    header_info: HashMap<String, String>,
}

pub fn start_multi_thread_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

pub fn start_single_thread_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let mut response = String::new();

    if !request.is_empty() {
        let request_info = parse_request(&request);
        response = match request_info.rest_method {
            REST::GET => {
                get_response_content(&request_info.request_path[..])
                //response = get_simple_response(&v[1])
            },
            REST::NONE | _ => {
                /* Don't do anything special here */
                get_response_content("")
            },
        };
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_request(request: &str) -> Request {
    let mut rest = REST::NONE;
    let mut path = String::new();
    let mut header = HashMap::new();

    if request.is_empty() {
        return Request {
            rest_method: rest,
            request_path: path,
            header_info: header,
        };
    }

    let lines = request.trim().lines();

    for (num, line) in lines.enumerate() {
        if num == 0 {
            let request_info: Vec<&str> = line.split_whitespace().collect();
            for (num, info) in request_info.iter().enumerate() {
                match num {
                    0 => {
                        rest = match &info[..] {
                            "GET" => REST::GET,
                            "PUT" => REST::PUT,
                            "POST" => REST::POST,
                            "DELETE" => REST::DELETE,
                            _ => REST::NONE,
                        };

                    },
                    1 => { path.push_str(info); },
                    _ => { /* Do nothing for now */ },
                };
            }
        } else {
            let header_info: Vec<&str> = line.splitn(2, ':').collect();
            if header_info.len() == 2 {
                header.insert(String::from(header_info[0]), String::from(header_info[1]));
            }
        }
    }

    return Request {
        rest_method: rest,
        request_path: path,
        header_info: header,
    };
}

fn get_simple_response(request: &str) -> String {
    let (resp_status, source_paths) =
        match &request[..] {
            "/" => get_resp_info(200),
            _ => get_resp_info(404),
        };

    let mut response = String::new();
    response.push_str(&resp_status);

    if !source_paths.is_empty() {
        for path in source_paths {
            if path.is_empty() { continue; }

            let mut file = File::open(&path).unwrap();
            let mut contents = String::new();

            file.read_to_string(&mut contents).unwrap();

            if contents.len() > 0 {
                response.push_str(&contents);
            } else {
                println!("Can't load contents!");
            }
        }
    }

    response
}

fn get_resp_info(status: u16) -> (String, Vec<String>) {
    let (resp_status, source_paths)  =
        match status {
            200 => {
                ("HTTP/1.1 200 OK\r\n\r\n",
                 vec![
                     get_source_path("index.html"),
                     get_source_path("styles.css"),
                     get_source_path("bundle.js"),
                     get_source_path("")
                 ])
            },
            _ => {
                ("HTTP/1.1 404 NOT FOUND\r\n\r\n",
                 vec![
                     get_source_path("404.html")
                 ])
            },
        };

    (String::from(resp_status), source_paths)
}

fn get_response_content(path: &str) -> String {

    let (status_line, path) =
        match &path[..] {
            "/" => (get_status(200), get_source_path("index.html")),
            "/styles.css" => (get_status(200), get_source_path("styles.css")),
            "/bundle.js" => (get_status(200), get_source_path("bundle.js")),
            "/favicon.ico" => (get_status(200), get_source_path("")),
            _ => (get_status(404), get_source_path("404.html")),
        };

    let mut response = String::new();
    response.push_str(&status_line);

    if !path.is_empty() {
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        if contents.len() > 0 {
            response.push_str(&contents);
        } else {
            println!("Can't load contents!");
        }
    }

    return response;
}

fn get_source_path(source: &str) -> String {

    let mut path = String::new();

    if !source.is_empty() {
        path.push_str("../client/public/");
        path.push_str(&source);
    }

    return path;
}

fn get_status(status: u16) -> String {
    let status_base =
        match status {
            200 => "200 OK",
            _ => "404 NOT FOUND"
        };

    return format!("HTTP/1.1 {}\r\n\r\n", status_base);
}