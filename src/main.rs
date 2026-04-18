use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use crate::backend::ThreadPool;
mod backend;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Faild bind");
    let pool = ThreadPool::new(4);

    // pool.execute(|| {
    //     println!("Hello from thread");
    // });

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Connection failed: {}", e);
                continue;
            }
        };
        println!("New cutomer login");

        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    // println!("Request: {} ",String::from_utf8_lossy(&buffer[..bytes_read]));
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    if let Some(line) = request.lines().next() {
        // println!("Request line: {}",line);
        let mut parts = line.split_whitespace();
        let method = parts.next();
        let path = parts.next();
        let (status_line, body) = match (method, path) {
            (Some("GET"), Some("/")) => ("HTTP/1.1 200 OK", "Home"),
            (Some("GET"), Some("/sleep")) => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "Sleep done")
            }
            (Some("POST"), Some("/")) => ("HTTP/1.1 200 OK", "Post received"),
            _ => ("HTTP/1.1 400 NOT FOUND", "Not found"),
        };

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            body.len(),
            body
        );
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Write Failed {}", e);
            return;
        }
        if let Err(e) = stream.flush() {
            eprintln!("Flush Failed {}", e);
        }
    }
}
