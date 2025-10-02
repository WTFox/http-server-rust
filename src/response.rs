use anyhow::Result;
use bytes::{Bytes, BytesMut};
use flate2::{write::GzEncoder, Compression};
use std::io::Write;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::{AppConfig, Headers, Request};

fn get_status(code: usize) -> &'static str {
    match code {
        200 => "200 OK",
        201 => "201 Created",
        404 => "404 Not Found",
        _ => unimplemented!(),
    }
}

pub async fn send_response(
    app: &AppConfig,
    request: &Request,
    response: &mut Response,
    stream: &mut TcpStream,
) -> Result<()> {
    let requested_encodings: Vec<String> = match request.headers.get("Accept-Encoding") {
        Some(enc) => enc.split(',').map(|n| n.trim().to_string()).collect(),
        None => vec![],
    };

    for requested_encoding in requested_encodings {
        if app.supported_encodings.contains(&requested_encoding) {
            response
                .headers
                .insert("Content-Encoding".to_string(), requested_encoding);
            break;
        }
    }

    match stream.write(&response.as_bytes()).await {
        Ok(_) => Ok(()),
        Err(e) => anyhow::bail!("couldn't send response {}", e),
    }
}

#[derive(Debug)]
pub struct Response {
    pub status_code: usize,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
}

impl Response {
    fn status_line(&self) -> String {
        format!("HTTP/1.1 {}\r\n", get_status(self.status_code))
    }

    fn headers(&self) -> String {
        let mut headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}\r\n", k, v))
            .collect::<Vec<String>>();

        headers.sort();
        headers.join("")
    }

    pub fn new(status_code: usize, headers: Headers, body: Option<Vec<u8>>) -> Response {
        Response {
            status_code: status_code,
            headers: headers,
            body: body,
        }
    }

    pub fn as_bytes(&mut self) -> Bytes {
        let mut output = BytesMut::new();

        output.extend_from_slice(&self.status_line().as_bytes());

        let wants_gzip = match self.headers.get("Content-Encoding").or(None) {
            Some(enc) => enc == &"gzip".to_string(),
            None => false,
        };

        let body_bytes = if wants_gzip {
            let mut e = GzEncoder::new(Vec::new(), Compression::default());
            e.write_all(self.body.as_deref().unwrap_or(b"")).unwrap();
            e.finish().unwrap()
        } else {
            self.body.as_deref().unwrap_or(b"").to_vec()
        };

        self.headers
            .insert("Content-Length".to_string(), body_bytes.len().to_string());

        if !self.headers().is_empty() {
            output.extend_from_slice(&self.headers().as_bytes());
            output.extend_from_slice(b"\r\n");
        }

        output.extend_from_slice(&body_bytes);
        output.freeze()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RESPONSE_BYTES: &[u8] =
        b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\nContent-Type: text/plain\r\n\r\nabc";

    #[test]
    fn test_as_bytes() {
        let mut response = Response::new(
            200,
            Headers::from([
                (String::from("Content-Type"), String::from("text/plain")),
                (String::from("Content-Length"), String::from("3")),
            ]),
            Some("abc".into()),
        );

        assert!(response.as_bytes() == RESPONSE_BYTES);
    }

    #[test]
    fn test_incomplete() {
        let mut response = Response::new(404, Headers::new(), None);
        assert!(*response.as_bytes() == *b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n");
    }

    #[test]
    fn test_with_headers() {
        let expected = Bytes::from("HTTP/1.1 200 OK\r\nContent-Length: 19\r\nContent-Type: text/plain\r\n\r\npineapple/raspberry");

        let mut response = Response::new(
            200,
            Headers::from([
                (String::from("Content-Type"), String::from("text/plain")),
                (String::from("Content-Length"), String::from("19")),
            ]),
            Some("pineapple/raspberry".into()),
        );
        assert!(response.as_bytes() == expected);
    }
}
