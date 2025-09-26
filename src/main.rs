use codecrafters_http_server::request::HttpMethod;
use codecrafters_http_server::{routes, Request};

use anyhow::Result;

use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn route_request(mut stream: TcpStream) -> Result<()> {
    let request = Request::from_stream(&stream)?;
    let path = request.path.as_str();
    match path {
        path if path == "/" => {
            routes::handle_index(&request, &mut stream)?;
        }
        path if path.starts_with("/user-agent") => {
            routes::handle_user_agent(&request, &mut stream)?;
        }
        path if path.starts_with("/echo/") => {
            routes::handle_echo(&request, &mut stream)?;
        }
        path if path.starts_with("/files/") => match request.method {
            HttpMethod::GET => routes::handle_file_get(&request, &mut stream)?,
            HttpMethod::POST => routes::handle_file_post(&request, &mut stream)?,
        },
        _ => {
            routes::handler_404(&request, &mut stream)?;
        }
    }
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    match route_request(stream) {
                        Err(e) => eprint!("error: {}", e),
                        _ => {}
                    };
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
