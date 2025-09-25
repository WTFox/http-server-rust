use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

fn handle_stream(mut stream: TcpStream) {
    println!("accepted new connection");
    let resp = b"HTTP/1.1 200 OK\r\n\r\n";
    stream.write(resp).unwrap();
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
