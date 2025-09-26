use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
}

impl From<&TcpStream> for Request {
    fn from(stream: &TcpStream) -> Self {
        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        let request_line = lines.next().unwrap().unwrap();
        let mut parts = request_line.split_whitespace();
        let method = parts.next().ok_or("Unable to parse method").unwrap();
        let path = parts.next().ok_or("Unable to parse path").unwrap();
        let protocol = parts.next().ok_or("Unable to parse protocol").unwrap();

        let mut headers = HashMap::new();

        while let Some(line) = lines.next() {
            let l = line.unwrap();
            if l.is_empty() {
                break;
            }
            let parts = l.split(": ").collect::<Vec<&str>>();
            let key = String::from(parts[0].trim());
            let value = String::from(parts[1].trim());
            headers.insert(key, value);
        }

        Request {
            method: method.into(),
            path: path.into(),
            protocol: protocol.into(),
            headers: headers,
        }
    }
}
