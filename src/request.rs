use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

use anyhow::Result;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

impl TryFrom<&str> for HttpMethod {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => anyhow::bail!("Invalid http method"),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn from_stream(stream: &TcpStream) -> Result<Request> {
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let mut parts = request_line.split_whitespace();

        let method = HttpMethod::try_from(
            parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing request method"))?,
        )?;

        let request_path = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing request path"))?;

        let protocol = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing protocol"))?;

        let mut headers = HashMap::new();
        loop {
            let mut header_line = String::new();
            reader.read_line(&mut header_line)?;

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

        Ok(Request {
            method: method,
            path: request_path.into(),
            protocol: protocol.into(),
            headers: headers,
            body: body,
        })
    }
}
