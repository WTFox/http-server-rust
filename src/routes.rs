use anyhow::Result;

use crate::{AppConfig, HttpMethod, Request};

use crate::response::send_response;
use crate::{Headers, Response};
use std::{fs, path::Path};
use tokio::net::TcpStream;

use std::io::Write;

pub async fn route_request(app: &AppConfig, mut stream: TcpStream) -> Result<()> {
    loop {
        let request = Request::from_stream(&mut stream).await?;
        let path = request.path.as_str();
        match path {
            path if path == "/" => {
                handle_index(&app, &request, &mut stream).await?;
            }
            path if path.starts_with("/user-agent") => {
                handle_user_agent(&app, &request, &mut stream).await?;
            }
            path if path.starts_with("/echo/") => {
                handle_echo(&app, &request, &mut stream).await?;
            }
            path if path.starts_with("/files/") => match request.method {
                HttpMethod::GET => handle_file_get(&app, &request, &mut stream).await?,
                HttpMethod::POST => handle_file_post(&app, &request, &mut stream).await?,
            },
            _ => {
                handler_404(&app, &request, &mut stream).await?;
            }
        }
    }
}

async fn handle_index(app: &AppConfig, request: &Request, stream: &mut TcpStream) -> Result<()> {
    send_response(
        app,
        request,
        &mut Response::new(
            200,
            Headers::from([(String::from("Content-Type"), String::from("text/plain"))]),
            None,
        ),
        stream,
    )
    .await
}

async fn handler_404(app: &AppConfig, request: &Request, stream: &mut TcpStream) -> Result<()> {
    send_response(
        app,
        request,
        &mut Response::new(404, Headers::new(), None),
        stream,
    )
    .await
}

async fn handle_echo(app: &AppConfig, request: &Request, stream: &mut TcpStream) -> Result<()> {
    let param = request.path.split("/").collect::<Vec<&str>>()[2];

    let requested_encoding = match request.headers.get("Accept-Encoding") {
        Some(enc) => enc.to_string(),
        None => "".to_string(),
    };
    let mut resp_headers = Headers::from([
        (String::from("Content-Type"), String::from("text/plain")),
        (String::from("Content-Length"), param.len().to_string()),
    ]);
    if app.supported_encodings.contains(&requested_encoding) {
        resp_headers.insert("Content-Encoding".to_string(), requested_encoding);
    }

    send_response(
        app,
        request,
        &mut Response::new(200, resp_headers, Some(param.into())),
        stream,
    )
    .await
}

async fn handle_user_agent(
    app: &AppConfig,
    request: &Request,
    stream: &mut TcpStream,
) -> Result<()> {
    let default_ua = String::new();
    let ua = request
        .headers
        .get("User-Agent")
        .or(Some(&default_ua))
        .unwrap();

    let mut body = None;
    if !ua.is_empty() {
        body = Some(ua.as_str().into());
    }

    send_response(
        app,
        request,
        &mut Response::new(
            200,
            Headers::from([
                (String::from("Content-Type"), String::from("text/plain")),
                (String::from("Content-Length"), ua.len().to_string()),
            ]),
            body,
        ),
        stream,
    )
    .await
}

async fn handle_file_get(app: &AppConfig, request: &Request, stream: &mut TcpStream) -> Result<()> {
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
        return send_response(
            app,
            request,
            &mut Response::new(
                200,
                Headers::from([
                    (
                        String::from("Content-Type"),
                        String::from("application/octet-stream"),
                    ),
                    (
                        String::from("Content-Length"),
                        file_contents.len().to_string(),
                    ),
                ]),
                Some(file_contents.into()),
            ),
            stream,
        )
        .await;
    }

    send_response(
        app,
        request,
        &mut Response::new(404, Headers::new(), None),
        stream,
    )
    .await
}

async fn handle_file_post(
    app: &AppConfig,
    request: &Request,
    stream: &mut TcpStream,
) -> Result<()> {
    let directory = app.directory.as_deref().unwrap_or_default();
    let filename = request.path.strip_prefix("/files/").unwrap();
    let contents = match request.body.as_ref() {
        Some(contents) => contents,
        None => "",
    };

    let filepath = format!("{}{}", directory, filename);
    let path = Path::new(&filepath);
    if path.exists() {
        return send_response(
            app,
            request,
            &mut Response::new(
                200,
                Headers::from([(
                    String::from("Content-Type"),
                    String::from("application/octet-stream"),
                )]),
                None,
            ),
            stream,
        )
        .await;
    }

    fs::create_dir_all(&directory)?;
    let mut f = fs::File::create(path)?;
    f.write_all(contents.as_bytes())?;

    send_response(
        app,
        request,
        &mut Response::new(201, Headers::new(), None),
        stream,
    )
    .await
}
