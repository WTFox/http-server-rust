use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

fn handle_stream(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("accepted new connection");

    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let mut parts = request_line.split_whitespace();
    let _method = parts.next().ok_or("Unable to parse method")?;
    let path = parts.next().ok_or("Unable to parse path")?;
    let _protocol = parts.next().ok_or("Unable to parse protocol")?;

    if path == "/" {
        println!("requesting index /");
        let resp = b"HTTP/1.1 200 OK\r\n\r\n";
        stream.write(resp)?;
    } else {
        println!("requesting path {}", path);
        let resp = b"HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write(resp)?;
    }
    Ok(())
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(stream).expect("error when handling incoming request");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
