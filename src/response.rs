#[derive(Debug)]
pub struct Response<'a> {
    pub status_line: &'a str,
    pub headers: Vec<&'a str>,
    pub body: &'a str,
}

impl<'a> From<&[u8]> for Response<'a> {
    fn from(_bytes: &[u8]) -> Response<'a> {
        todo!();
    }
}

impl<'a> Response<'a> {
    pub fn new(status_line: &'a str, headers: Vec<&'a str>, body: &'a str) -> Response<'a> {
        Response {
            status_line: status_line,
            headers: headers,
            body: body,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        format!(
            "{}\r\n{}\r\n\r\n{}",
            self.status_line,
            self.headers.join("\r\n"),
            self.body
        )
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RESPONSE_BYTES: &[u8] =
        b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\nabc";

    #[test]
    fn test_as_bytes() {
        let response = Response::new(
            "HTTP/1.1 200 OK",
            vec!["Content-Type: text/plain", "Content-Length: 3"],
            "abc",
        );

        assert!(&response.as_bytes() == RESPONSE_BYTES);
    }
}
