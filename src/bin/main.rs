// Bring in libraries for networking, I/O, threading, and files
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::Error;
use std::thread;
use std::time::Duration;
use std::fs;

use server::ThreadPool;

// Start the web server
fn main() {
    // Listen for connections on localhost:7878
    let listener: TcpListener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();

        // Create a pool of 4 worker threads
        let pool = ThreadPool::new(4);

        // Accept incoming connections
        for stream in listener.incoming() {
            let stream: TcpStream = stream.unwrap();

            // Send each connection to a worker thread
            pool.execute(|| {
                handle_connection(stream);
            });
        }

        // Cleanup message
        print!("Shutting down.\n");
}

// Process an HTTP request
fn handle_connection(mut stream: TcpStream) {
    // Read the incoming request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Received request: {}", String::from_utf8_lossy(&buffer));

    // Request types we support
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Decide which page to send
    let (status_line, filename) = 
        if buffer.starts_with(get) {
            // Home page
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep) {
            // Slow endpoint
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            // Not found
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    // Read the file
    let contents = 
        fs::read_to_string(filename).unwrap();

    // Build the response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    // Send the response
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}