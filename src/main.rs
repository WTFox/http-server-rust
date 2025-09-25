use codecrafters_http_server::{Request, Response};

use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

fn handle_index(_request: &Request, stream: &mut TcpStream) {
    let resp = b"HTTP/1.1 200 OK\r\n\r\n";
    stream.write(resp).expect("uh oh");
}

fn handler_404(_request: &Request, stream: &mut TcpStream) {
    let resp = b"HTTP/1.1 404 Not Found\r\n\r\n";
    stream.write(resp).expect("uh oh");
}

fn handle_echo(request: &Request, stream: &mut TcpStream) {
    let parts: Vec<&str> = request.path.split("/").collect();
    let param = parts[2];

    let header = format!("Content-Length: {}", &param.len());
    let resp = Response::new(
        "HTTP/1.1 200 OK",
        vec!["Content-Type: text/plain", &header],
        param,
    );
    stream.write(&resp.as_bytes()).expect("uh oh");
}

fn handle_stream(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("accepted new connection");
    let request = Request::from(&stream);
    let path = request.path.as_str();
    match path {
        path if path == "/" => {
            handle_index(&request, &mut stream);
        }
        path if path.starts_with("/echo/") => {
            handle_echo(&request, &mut stream);
        }
        _ => {
            handler_404(&request, &mut stream);
        }
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
