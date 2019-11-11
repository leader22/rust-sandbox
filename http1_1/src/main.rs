use serde_json::json;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::TcpListener;

mod parser;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // XXX: should be done by thread

        let mut reader = BufReader::new(&stream);
        let json = match parser::parse_request(&mut reader) {
            Some(req) => json!(req),
            None => json!({ "err": "invalid request" }),
        };

        let mut writer = BufWriter::new(&stream);
        let resp = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", json);
        writer.write(resp.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
