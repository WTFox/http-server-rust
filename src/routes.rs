use crate::{Request, Response};
use std::{env, fs, path::Path};

use std::{io::Write, net::TcpStream};

pub fn handle_index(_request: &Request, stream: &mut TcpStream) {
    let resp = Response::new(200, vec![], None);
    println!("responding with: {:?}", resp.as_bytes());
    stream.write(&resp.as_bytes()).expect("uh oh");
}

pub fn handler_404(_request: &Request, stream: &mut TcpStream) {
    let resp = Response::new(404, vec![], None);
    println!("responding with: {:?}", resp.as_bytes());
    stream.write(&resp.as_bytes()).expect("uh oh");
}

pub fn handle_echo(request: &Request, stream: &mut TcpStream) {
    let param = request.path.split("/").collect::<Vec<&str>>()[2];
    let header = format!("Content-Length: {}", &param.len());
    let resp = Response::new(200, vec!["Content-Type: text/plain", &header], Some(param));
    println!("responding with: {:?}", resp.as_bytes());
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
    println!("responding with: {:?}", resp.as_bytes());
    stream.write(&resp.as_bytes()).expect("uh oh");
}

pub fn handle_file_get(request: &Request, stream: &mut TcpStream) {
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
    println!("responding with: {:?}", resp.as_bytes());
    stream.write(&resp.as_bytes()).expect("uh oh");
}

pub fn handle_file_post(request: &Request, stream: &mut TcpStream) {
    println!("handle_file_post");

    let args: Vec<String> = env::args().collect();
    let mut directory = String::from("");
    if args.len() == 3 && args[1] == "--directory" {
        directory = args[2].clone();
    }
    println!("args: {:?}", args);
    println!("post: request.path: {:?}", request.path);

    let filename = request
        .path
        .strip_prefix("/files/")
        .expect("invalid filepath");

    println!("filename: {:?}", filename);

    println!("creating dirs");
    println!("dir: {:?}", directory);
    fs::create_dir_all(&directory).expect("OH GOD");

    let filepath = format!("{}{}", directory, filename);
    println!("filepath: {:?}", filepath);
    let path = Path::new(&filepath);
    println!("path: {:?}", path);

    if path.exists() {
        println!("path exists");
        panic!("uhoh file already there??");
    }

    let mut f = fs::File::create(path).expect("couldn't create file");
    println!("file created successfully");
    let contents = match request.body.as_ref() {
        Some(contents) => contents,
        None => "",
    };
    println!("contents: {:?}", contents);
    f.write_all(contents.as_bytes())
        .expect("couldn't write file?");

    let resp = Response {
        status_code: 201,
        headers: vec![],
        body: None,
    };
    println!("resp: {:?}", resp);
    println!("responding with: {:?}", resp.as_bytes());
    stream.write(&resp.as_bytes()).expect("uh oh");
}
