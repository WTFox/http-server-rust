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
    pub headers: Vec<&'a str>,
    pub body: Option<&'a str>,
}

impl<'a> From<&[u8]> for Response<'a> {
    fn from(_bytes: &[u8]) -> Response<'a> {
        todo!();
    }
}

impl<'a> Response<'a> {
    fn status_line(&self) -> String {
        format!("HTTP/1.1 {}\r\n", get_status(self.status_code))
    }

    pub fn new(status_code: usize, headers: Vec<&'a str>, body: Option<&'a str>) -> Response<'a> {
        Response {
            status_code: status_code,
            headers: headers,
            body: body,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut output = String::new();
        output.push_str(&self.status_line().as_str());

        println!("1. building response bytes");
        if !self.headers.is_empty() {
            output.push_str(self.headers.join("\r\n").as_str());
            output.push_str("\r\n");
        }
        output.push_str("\r\n");

        println!("building response bytes");
        match &self.body {
            Some(body) => {
                output.push_str(body);
            }
            _ => {
                output.push_str("\r\n");
            }
        }
        println!("response vect: {:?}", output);
        output.into()
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
            200,
            vec!["Content-Type: text/plain", "Content-Length: 3"],
            Some("abc"),
        );

        assert!(&response.as_bytes() == RESPONSE_BYTES);
    }

    #[test]
    fn test_incomplete() {
        let response = Response::new(404, vec![], None);
        assert!(&response.as_bytes() == b"HTTP/1.1 404 Not Found\r\n\r\n");
    }

    #[test]
    fn test_with_headers() {
        let expected = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 19\r\n\r\npineapple/raspberry";

        let response = Response {
            status_code: 200,
            headers: vec!["Content-Type: text/plain", "Content-Length: 19"],
            body: Some("pineapple/raspberry"),
        };
        assert!(&response.as_bytes() == expected);
    }
}
