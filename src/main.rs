// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                thread::spawn(|| handle_client(_stream));
                // handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 7];
    let mut buf2 = [0; 20];
    buf.copy_from_slice(b"+PONG\r\n");
    loop {
        let read_bytes = stream.read(&mut buf2).expect("Failed to read");
        if (read_bytes == 0) {
            return;
        }
        stream
            .write_all(&buf[0..7])
            .expect("Failed to write to client");
    }
}
