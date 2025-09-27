use codecrafters_http_server::{routes, AppConfig, HttpMethod, Request};

use anyhow::Result;

use clap::Parser;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    directory: Option<String>,
}

fn route_request(app: &AppConfig, mut stream: TcpStream) -> Result<()> {
    let request = Request::from_stream(&stream)?;
    let path = request.path.as_str();
    match path {
        path if path == "/" => {
            routes::handle_index(&app, &request, &mut stream)?;
        }
        path if path.starts_with("/user-agent") => {
            routes::handle_user_agent(&app, &request, &mut stream)?;
        }
        path if path.starts_with("/echo/") => {
            routes::handle_echo(&app, &request, &mut stream)?;
        }
        path if path.starts_with("/files/") => match request.method {
            HttpMethod::GET => routes::handle_file_get(&app, &request, &mut stream)?,
            HttpMethod::POST => routes::handle_file_post(&app, &request, &mut stream)?,
        },
        _ => {
            routes::handler_404(&app, &request, &mut stream)?;
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    let app = AppConfig {
        directory: args.directory,
    };

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let app_clone = app.clone();
                thread::spawn(move || {
                    match route_request(&app_clone, stream) {
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
