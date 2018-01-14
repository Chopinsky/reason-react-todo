pub mod thread_utils;
pub mod connection;

use std::env;
use connection::start_single_thread_server;
use connection::start_multi_thread_server;

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

    if on_single_thread {
        println!("\nStarting single-thread server...\n");
        start_single_thread_server();
    } else {
        println!("\nStarting multi-thread server...\n");
        start_multi_thread_server();
    }
}
