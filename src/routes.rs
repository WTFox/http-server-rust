use anyhow::Result;

use crate::{AppConfig, Headers, Request, Response};
use std::{fs, path::Path};

use std::{io::Write, net::TcpStream};

pub fn handle_index(_app: &AppConfig, _request: &Request, stream: &mut TcpStream) -> Result<()> {
    Response::new(200, Headers::new(), None).send(stream)
}

pub fn handler_404(_app: &AppConfig, _request: &Request, stream: &mut TcpStream) -> Result<()> {
    Response::new(404, Headers::new(), None).send(stream)
}

pub fn handle_echo(_app: &AppConfig, request: &Request, stream: &mut TcpStream) -> Result<()> {
    let param = request.path.split("/").collect::<Vec<&str>>()[2];
    Response::new(
        200,
        Headers::from([
            (String::from("Content-Type"), String::from("text/plain")),
            (String::from("Content-Length"), param.len().to_string()),
        ]),
        Some(param),
    )
    .send(stream)
}

pub fn handle_user_agent(
    _app: &AppConfig,
    request: &Request,
    stream: &mut TcpStream,
) -> Result<()> {
    let default_ua = String::from("");
    let ua = request
        .headers
        .get("User-Agent")
        .or(Some(&default_ua))
        .unwrap();

    let mut body = None;
    if !ua.is_empty() {
        body = Some(ua.as_str())
    }

    Response::new(
        200,
        Headers::from([
            (String::from("Content-Type"), String::from("text/plain")),
            (String::from("Content-Length"), ua.len().to_string()),
        ]),
        body,
    )
    .send(stream)
}

pub fn handle_file_get(app: &AppConfig, request: &Request, stream: &mut TcpStream) -> Result<()> {
    let filename = request
        .path
        .strip_prefix("/files/")
        .expect("invalid filepath");

    let filepath = format!(
        "{}{}",
        app.directory.as_deref().unwrap_or_default(),
        filename
    );
    let path = Path::new(&filepath);

    if path.exists() {
        let file_contents = fs::read_to_string(path)?;
        let resp = Response {
            status_code: 200,
            headers: Headers::from([
                (
                    String::from("Content-Type"),
                    String::from("application/octet-stream"),
                ),
                (
                    String::from("Content-Length"),
                    file_contents.len().to_string(),
                ),
            ]),
            body: Some(file_contents.as_str()),
        };
        stream.write(&resp.as_bytes())?;
        return Ok(());
    }

    Response::new(404, Headers::new(), None).send(stream)
}

pub fn handle_file_post(app: &AppConfig, request: &Request, stream: &mut TcpStream) -> Result<()> {
    let directory = app.directory.as_deref().unwrap_or_default();
    let filename = request.path.strip_prefix("/files/").unwrap();
    let contents = match request.body.as_ref() {
        Some(contents) => contents,
        None => "",
    };

    let filepath = format!("{}{}", directory, filename);
    let path = Path::new(&filepath);
    if path.exists() {
        println!("path exists");
        panic!("uhoh file already there??");
    }

    fs::create_dir_all(&directory)?;
    let mut f = fs::File::create(path)?;
    f.write_all(contents.as_bytes())?;

    Response::new(201, Headers::new(), None).send(stream)
}
