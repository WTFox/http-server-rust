use crate::{Request, Response};
use std::{env, fs, path::Path};

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

pub fn handle_file(request: &Request, stream: &mut TcpStream) {
    let args: Vec<String> = env::args().collect();
    let mut directory = String::from("");
    if args.len() == 3 && args[1] == "--directory" {
        directory = args[2].clone();
    }

    let filename = request
        .path
        .strip_prefix("/files/")
        .expect("invalid filepath");

    let filepath = format!("{}{}", directory, filename);
    let path = Path::new(&filepath);

    if path.exists() {
        let file_contents = fs::read_to_string(path).expect("no file found");

        let mut headers = vec!["Content-Type: application/octet-stream"];
        let new = format!("Content-Length: {}", file_contents.len());
        headers.push(&new);

        let resp = Response {
            status_code: 200,
            headers: headers,
            body: Some(file_contents.as_str()),
        };
        stream.write(&resp.as_bytes()).expect("uh oh");
        return;
    }

    let resp = Response {
        status_code: 404,
        headers: vec![],
        body: None,
    };
    stream.write(&resp.as_bytes()).expect("uh oh");
}
