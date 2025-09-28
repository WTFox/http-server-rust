use codecrafters_http_server::{routes, AppConfig};

use clap::Parser;
use std::{net::TcpListener, thread};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    directory: Option<String>,
}

fn main() {
    let args = Args::parse();
    let app = AppConfig::new(args.directory);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let app_clone = app.clone();
                thread::spawn(move || {
                    match routes::route_request(&app_clone, stream) {
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
