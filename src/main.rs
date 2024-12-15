use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::thread;

fn handle_client(mut stream: TcpStream, port: String) {
    let mut buffer = [0; 1024];

    // Read the request from the client
    match stream.read(&mut buffer) {
        Ok(n) if n > 0 => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("BE: Received request: {}", request);

            // Send a simple response back to the client
            let response = format!("HTTP/1.1 200 OK Content:Hello from Backend Server! Port: {}\n", port);
            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("BE: Failed to send response: {}", e);
            }
        },
        _ => {
            eprintln!("BE: Failed to read request or request was empty");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: backend <port>");
        process::exit(1);
    }

    let port = args[1].clone(); // Clone the port string into a new owned String
    let addr = format!("127.0.0.1:{}", port);

    // Start listening on the specified port
    let listener = TcpListener::bind(&addr).expect("BE: Failed to bind to address");
    println!("Backend server Listening on {}", addr);

    // Handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle each client connection in a new thread
                let port = port.clone(); // Clone `port` here for each thread
                thread::spawn(move || handle_client(stream, port)); // Move `port` into the closure
            },
            Err(e) => {
                eprintln!("BE: Failed to accept connection: {}", e);
            }
        }
    }
}
