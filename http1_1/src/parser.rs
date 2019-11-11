use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

pub fn parse_request(reader: &mut BufReader<&TcpStream>) -> Option<Request> {
    // read first line for request
    let mut first_line = String::new();
    match reader.read_line(&mut first_line) {
        Ok(_) => (),
        Err(_) => return None,
    }
    let mut request = parse_request_line(&first_line)?;

    // read until empty line for headers
    // rest is message body
    loop {
        let mut next_line = String::new();
        match reader.read_line(&mut next_line) {
            Ok(_) => (),
            Err(_) => return None,
        }

        if next_line == "\r\n" {
            break;
        }

        match parse_header_line(&next_line) {
            Some(header) => request.headers.push(header),
            None => (),
        }
    }

    // XXX: should check Content-Type header
    // if message body exists
    let mut buf = reader.buffer();
    if buf.len() > 0 {
        match buf.read_to_string(&mut request.body) {
            Ok(_) => (),
            Err(_) => return None,
        }
    }

    Some(request)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    method: String,
    path: String,
    version: String,
    headers: Vec<Header>,
    body: String,
}
fn parse_request_line(line: &str) -> Option<Request> {
    let mut req = line.trim().split_whitespace();
    let (method, path, version) = (req.next(), req.next(), req.next());
    match (method, path, version) {
        (Some(method), Some(path), Some(version)) => Some(Request {
            method: method.to_string(),
            path: path.to_string(),
            version: version.to_string(),
            headers: Vec::new(),
            body: "".to_string(),
        }),
        _ => None,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Header {
    key: String,
    value: String,
}

fn parse_header_line(line: &str) -> Option<Header> {
    let kv: Vec<&str> = line.trim().split(":").collect();
    match (kv.get(0), kv.get(1)) {
        (Some(key), Some(value)) => Some(Header {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prl_should_return_request() {
        let req = parse_request_line("GET / HTTP/1.1");
        match req {
            Some(req) => {
                assert_eq!(req.method, "GET");
                assert_eq!(req.path, "/");
                assert_eq!(req.version, "HTTP/1.1");
            }
            None => assert!(false, "should not happen"),
        }

        let req = parse_request_line("POST /foo/bar HTTP/0.9");
        match req {
            Some(req) => {
                assert_eq!(req.method, "POST");
                assert_eq!(req.path, "/foo/bar");
                assert_eq!(req.version, "HTTP/0.9");
            }
            None => assert!(false, "should not happen"),
        }
    }

    #[test]
    fn prl_should_return_none() {
        assert!(parse_request_line("").is_none(), "for empty string");
        assert!(parse_request_line("GET /").is_none(), "for invalid string");
        assert!(parse_request_line("/").is_none(), "for invalid string");
    }

    #[test]
    fn phl_should_return_header() {
        let header = parse_header_line("Content-Length: 30");
        match header {
            Some(hd) => {
                assert_eq!(hd.key, "Content-Length");
                assert_eq!(hd.value, "30");
            }
            None => assert!(false, "should not happen"),
        }

        let header = parse_header_line("Accept : */*\r\n");
        match header {
            Some(hd) => {
                assert_eq!(hd.key, "Accept");
                assert_eq!(hd.value, "*/*");
            }
            None => assert!(false, "should trim"),
        }
    }

    #[test]
    fn phl_should_return_none() {
        assert!(parse_header_line("").is_none(), "for empty string");
        assert!(parse_header_line("Foo").is_none(), "for invalid string");
    }
}
