use super::*;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::thread;

#[test]
fn requests() {
    assert_eq!(
        build_request("GET", "example.com", "/", ""),
        "GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n"
    );
    assert_eq!(
        build_request("POST", "api.local", "/items", "{\"a\":1}"),
        "POST /items HTTP/1.1\r\nHost: api.local\r\nConnection: close\r\nContent-Length: 7\r\n\r\n{\"a\":1}"
    );
}

#[test]
fn parsing() {
    let raw = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\ncontent-length:  5\r\n\r\nnope!extra";
    let r = parse_response(raw).unwrap();
    assert_eq!(r.status, 404);
    assert_eq!(r.reason, "Not Found");
    assert_eq!(r.header("Content-Length"), Some("5"));
    assert_eq!(r.header("content-type"), Some("text/plain"));
    assert_eq!(r.header("X-Missing"), None);
    assert_eq!(r.body, "nope!", "Content-Length limits the body");

    let r = parse_response("HTTP/1.0 200 OK\r\n\r\nall of it").unwrap();
    assert_eq!(r.body, "all of it");
    assert!(r.headers.is_empty());
}

#[test]
fn parse_errors() {
    assert!(parse_response("HTTP/1.1 200 OK\r\nNo-Blank-Line: yes").is_err());
    assert!(parse_response("HTTP/1.1 abc OK\r\n\r\n").is_err());
    assert!(parse_response("SPDY 200 OK\r\n\r\n").is_err());
    assert!(parse_response("HTTP/1.1 200 OK\r\nbroken header\r\n\r\n").is_err());
}

#[test]
fn fetch_from_local_server() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(&stream);
        let mut first = String::new();
        reader.read_line(&mut first).unwrap();
        loop {
            let mut h = String::new();
            reader.read_line(&mut h).unwrap();
            if h == "\r\n" || h.is_empty() {
                break;
            }
        }
        let body = format!("path={}", first.split(' ').nth(1).unwrap());
        let mut s = &stream;
        write!(s, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nServer: test\r\n\r\n{}", body.len(), body).unwrap();
    });
    let r = fetch(addr, "/status").unwrap();
    server.join().unwrap();
    assert_eq!(r.status, 200);
    assert_eq!(r.body, "path=/status");
    assert_eq!(r.header("server"), Some("test"));
}
