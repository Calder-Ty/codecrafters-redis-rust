use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::vec;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

enum RedisCommand {
    Ping,
}

fn handle_client(mut stream: TcpStream) {
    let mut input = [0; 512];
    let _ = stream.read(input.as_mut_slice());
    for cmd in parse_client_input(&input) {
        match cmd {
            RedisCommand::Ping => {
                let _ = stream.write_all(b"+PONG\n\r");
            },
        };
    }
}

fn parse_client_input(input: &[u8]) -> impl Iterator<Item = RedisCommand> {
    let mut outs = vec![];
    let mut cmd_buff = vec![];
    for byte in input {
        if *byte == b'\n' {
            let cmd = RedisCommand::Ping;
            outs.push(cmd);
            cmd_buff.clear();
        };
        cmd_buff.push(byte);
    }
    outs.into_iter()
}
