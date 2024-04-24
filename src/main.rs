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
    let mut buf2 = [0; 1024];

    loop {
        let read_bytes = stream.read(&mut buf2).expect("Failed to read");
        if (read_bytes == 0) {
            return;
        }
        let s = String::from_utf8_lossy(&buf2[0..read_bytes]);

        let ans = handle_request(&s.to_string());
        let buf = ans.as_bytes();
        stream
            .write_all(&buf[0..buf.len()])
            .expect("Failed to write to client");
    }
}
fn handle_request(req: &String) -> String {
    let echo_pattern = "echo\r\n";
    if let Some(pos) = req.find(echo_pattern) {
        let extracted_chars = &req[pos + echo_pattern.len()..];
        extracted_chars.to_string()
    } else {
        "+PONG\r\n".to_string()
    }
}
