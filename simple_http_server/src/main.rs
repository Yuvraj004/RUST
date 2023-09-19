use std::io::{Read, Write};//Imports the necessary traits for reading and writing.
use std::net::{TcpListener, TcpStream};// Imports the necessary types for networking.
use threadpool::ThreadPool;//optimizing the server for handling a large no. of requests

//Defines a function to handle an incoming client connection. It reads the incoming data, sends a basic HTTP response, and flushes the stream.
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Read error");

    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
    stream.write(response.as_bytes()).expect("Write error");
    stream.flush().expect("Flush error");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Listener failed");//Binds a TCP listener to 127.0.0.1:8080.
    let pool = ThreadPool::new(4);// Create a thread pool with 4 threads

    println!("Server is running at http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            //handles the result of accepting a connection. If successful, it spawns a new thread to handle the client.
            // Ok(stream) => {
            //     std::thread::spawn(|| {
            //         handle_client(stream);
            //     });
            // }
            Ok(stream) => {
                pool.execute(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
