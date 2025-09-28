use anyhow::Result;
use bytes::{Bytes, BytesMut};
use std::{io::Write, net::TcpStream};

use crate::Headers;

fn get_status(code: usize) -> &'static str {
    match code {
        200 => "200 OK",
        201 => "201 Created",
        404 => "404 Not Found",
        _ => unimplemented!(),
    }
}

#[derive(Debug)]
pub struct Response<'a> {
    pub status_code: usize,
    pub headers: Headers,
    pub body: Option<&'a str>,
}

impl<'a> Response<'a> {
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
        headers.concat()
    }

    pub fn send(&self, request: &Request, stream: &mut TcpStream) -> Result<()> {
        match stream.write(&self.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => anyhow::bail!("couldn't send response {}", e),
        }
    }

    pub fn new(status_code: usize, headers: Headers, body: Option<&'a str>) -> Response<'a> {
        Response {
            status_code: status_code,
            headers: headers,
            body: body,
        }
    }

    pub fn as_bytes(&self) -> Bytes {
        let mut output = BytesMut::new();

        output.extend_from_slice(&self.status_line().as_bytes());
        if !self.headers.is_empty() {
            output.extend_from_slice(&self.headers().as_bytes());
            output.extend_from_slice(b"\r\n");
        }
        output.extend_from_slice(self.body.as_deref().unwrap_or("\r\n").as_bytes());

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
        let response = Response::new(
            200,
            Headers::from([
                (String::from("Content-Type"), String::from("text/plain")),
                (String::from("Content-Length"), String::from("3")),
            ]),
            Some("abc"),
        );

        assert!(&response.as_bytes() == RESPONSE_BYTES);
    }

    #[test]
    fn test_incomplete() {
        let response = Response::new(404, Headers::new(), None);
        assert!(*response.as_bytes() == *b"HTTP/1.1 404 Not Found\r\n\r\n");
    }

    #[test]
    fn test_with_headers() {
        let expected = b"HTTP/1.1 200 OK\r\nContent-Length: 19\r\nContent-Type: text/plain\r\n\r\npineapple/raspberry";

        let response = Response::new(
            200,
            Headers::from([
                (String::from("Content-Type"), String::from("text/plain")),
                (String::from("Content-Length"), String::from("19")),
            ]),
            Some("pineapple/raspberry"),
        );
        assert!(*response.as_bytes() == *expected);
    }
}
