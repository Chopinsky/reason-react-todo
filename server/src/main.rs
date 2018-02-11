extern crate rusty_express;
use rusty_express::prelude::*;

//extern crate http_server;
//use http_server::prelude::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let on_single_thread =
        match args.len() {
            2 => {
                let version = &args[1];
                match &version[..] {
                    "--single-thread" => true,
                    "-S" => true,
                    _ => false,
                }
            },
            _ => false,
        };

    let pool_size: usize = if on_single_thread {
        println!("\nStarting single-thread server...\n");
        1
    } else {
        println!("\nStarting multi-thread server...\n");
        8
    };


    let mut server = HttpServer::new();
    server.set_pool_size(pool_size);

    server.get(RequestPath::WildCard(r"^/\w*?"), main_handler);

    server.listen(8080);
}

fn main_handler(req: &Request, resp: &mut Response) {

//    println!("Ready to server: {}", req.path);

    match &req.uri[..] {
        "/" => {
            resp.send_file("../client/public/index.html");
            resp.status(200);
        },
        "/bundle.js" => {
            resp.send_file("../client/public/bundle.js");
            resp.status(200);
        },
        "/styles.css" => {
            resp.send_file("../client/public/styles.css");
            resp.status(200);
        },
        _ => {
            resp.status(404);
        }
    }
}
