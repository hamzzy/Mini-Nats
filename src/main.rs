use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use rand::prelude::*;

type Subscribers<T> = HashMap<String, Vec<T>>;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4222").expect("Failed to bind to address");
    println!("server running on 127.0.0.1:4222");
    let subscribers: Subscribers<TcpStream> = HashMap::new();
    let queue_groups: Subscribers<HashSet<TcpStream>> = HashMap::new();
    let state = Arc::new(Mutex::new((subscribers, queue_groups)));

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        let state = Arc::clone(&state);
        thread::spawn(move || handle_connection(stream, state));
    }

    
}

fn handle_connection(mut stream: TcpStream, state: Arc<Mutex<(Subscribers<TcpStream>, Subscribers<HashSet<TcpStream>>)>>) {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
        if bytes_read == 0 {
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut parts = message.split_whitespace();
        let command = parts.next().unwrap();

        match command {
            "PUB" => {
                let subject = parts.next().expect("Missing subject");
                let payload = parts.next().expect("Missing payload");
                eprintln!("{}", payload)
            }
            "SUB" => {
                let subject = parts.next().expect("Missing subject");
                eprintln!("{}", subject)
            }
           
            _ => {
                eprintln!("Unknown command: {}", command);
            }
        }
    }
}
