use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl From<&TcpStream> for Request {
    fn from(stream: &TcpStream) -> Self {
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line).expect("uh oh");

        let mut parts = request_line.split_whitespace();

        let method = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing HTTP method"))
            .expect("uh oh");

        let request_path = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing request path"))
            .expect("uh oh");

        let mut headers = HashMap::new();
        loop {
            let mut header_line = String::new();
            reader.read_line(&mut header_line).expect("Uh oh");

            let header_line = header_line.trim();
            if header_line.is_empty() {
                break;
            }
            if let Some((name, value)) = header_line.split_once(":") {
                headers.insert(name.trim().to_string(), value.trim().to_string());
            }
        }

        let body_length: usize = match headers.get("Content-Length") {
            Some(length) => length.parse().unwrap_or(0),
            None => 0,
        };

        let body = if body_length > 0 {
            let mut body_buf = vec![0u8; body_length];
            match reader.read_exact(&mut body_buf) {
                Ok(_) => String::from_utf8(body_buf).ok(),
                Err(_) => None,
            }
        } else {
            None
        };

        Request {
            method: method.into(),
            path: request_path.into(),
            protocol: String::from("HTTP 1.1"),
            headers: headers,
            body: body,
        }
    }
}
