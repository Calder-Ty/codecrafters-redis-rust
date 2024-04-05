use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => loop {
                let mut input = [0; 512];
                let read_count = _stream
                    .read(input.as_mut_slice())
                    .expect("Could Not Read from Stream");
                if read_count == 0 {
                    break;
                }
                let _ = _stream.write_all(b"+PONG\r\n");
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
