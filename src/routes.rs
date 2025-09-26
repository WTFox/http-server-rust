use crate::{Request, Response};

use std::{io::Write, net::TcpStream};

pub fn handle_index(_request: &Request, stream: &mut TcpStream) {
    let resp = Response::new(200, vec![], None);
    stream.write(&resp.as_bytes()).expect("uh oh");
}

pub fn handler_404(_request: &Request, stream: &mut TcpStream) {
    let resp = Response::new(404, vec![], None);
    stream.write(&resp.as_bytes()).expect("uh oh");
}

pub fn handle_echo(request: &Request, stream: &mut TcpStream) {
    let param = request.path.split("/").collect::<Vec<&str>>()[2];
    let header = format!("Content-Length: {}", &param.len());
    let resp = Response::new(200, vec!["Content-Type: text/plain", &header], Some(param));
    stream.write(&resp.as_bytes()).expect("uh oh");
}

pub fn handle_user_agent(request: &Request, stream: &mut TcpStream) {
    let ua = request
        .headers
        .get("User-Agent")
        .expect("No user agent was provided");

    let mut headers = vec!["Content-Type: text/plain"];

    let ua_len = ua.len().to_string();
    let new = format!("Content-Length: {}", ua_len);
    headers.push(&new);

    let mut body = None;
    if !ua.is_empty() {
        body = Some(ua.as_str())
    }

    let resp = Response::new(200, headers, body);
    stream.write(&resp.as_bytes()).expect("uh oh");
}
