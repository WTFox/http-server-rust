use std::io::{BufRead, BufReader};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub host: Option<String>,
    pub user_agent: Option<String>,
    pub accept: Option<String>,
}

impl From<&TcpStream> for Request {
    fn from(stream: &TcpStream) -> Self {
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        reader
            .read_line(&mut request_line)
            .expect("Can't read line");

        let mut parts = request_line.split_whitespace();
        let _method = parts.next().ok_or("Unable to parse method").unwrap();
        let path = parts.next().ok_or("Unable to parse path").unwrap();
        let _protocol = parts.next().ok_or("Unable to parse protocol").unwrap();

        Request {
            method: _method.into(),
            path: path.into(),
            protocol: _protocol.into(),
            host: None,
            user_agent: None,
            accept: None,
        }
    }
}
