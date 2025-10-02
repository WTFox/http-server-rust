use codecrafters_http_server::{routes, AppConfig};

use clap::Parser;
use tokio::net::TcpListener;

use anyhow::Result;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    directory: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let app = AppConfig::new(args.directory);

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await?;

        let app_clone = app.clone();
        tokio::spawn(async move {
            match routes::route_request(&app_clone, stream).await {
                Ok(_) => {}
                Err(_) => {}
            }
        });
    }
}
