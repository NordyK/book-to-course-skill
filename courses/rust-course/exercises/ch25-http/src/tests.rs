use super::*;
use std::io::Read;
use std::net::TcpListener;
use std::thread;

fn parse(raw: &str) -> Option<Request> {
    read_request(BufReader::new(raw.as_bytes()))
}

fn get(path: &str) -> Request {
    Request { method: "GET".into(), path: path.into(), headers: vec![] }
}

#[test]
fn parses_request() {
    let req = parse("GET /health HTTP/1.1\r\nHost: localhost\r\nUser-Agent:  test \r\n\r\nignored body").unwrap();
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/health");
    assert_eq!(req.headers.len(), 2);
    assert_eq!(req.header("host"), Some("localhost"));
    assert_eq!(req.header("USER-AGENT"), Some("test"));
    assert_eq!(req.header("accept"), None);
}

#[test]
fn bare_newlines_and_eof() {
    let req = parse("POST /form HTTP/1.0\nHost: x\n").unwrap();
    assert_eq!(req.method, "POST");
    assert_eq!(req.header("Host"), Some("x"));
}

#[test]
fn bad_requests() {
    for raw in ["", "\r\n", "GET /\r\n\r\n", "GET / FTP/1.0\r\n\r\n", "GET / HTTP/1.1 extra\r\n\r\n"] {
        assert_eq!(parse(raw), None, "{raw:?}");
    }
}

#[test]
fn skips_malformed_headers() {
    let req = parse("GET / HTTP/1.1\r\nno colon here\r\nX-A: 1:2\r\n\r\n").unwrap();
    assert_eq!(req.headers, vec![("X-A".to_string(), "1:2".to_string())]);
}

#[test]
fn reasons_and_format() {
    assert_eq!(reason(200), "OK");
    assert_eq!(reason(400), "BAD REQUEST");
    assert_eq!(reason(404), "NOT FOUND");
    assert_eq!(reason(405), "METHOD NOT ALLOWED");
    assert_eq!(reason(418), "");
    let r = Response { status: 200, body: "héllo".into() };
    assert_eq!(r.to_http(), "HTTP/1.1 200 OK\r\nContent-Length: 6\r\nContent-Type: text/html\r\n\r\nhéllo");
}

#[test]
fn routing() {
    assert_eq!(route(&get("/")), Response { status: 200, body: HELLO_HTML.into() });
    assert_eq!(route(&get("/?x=1")).status, 200);
    assert_eq!(route(&get("/health")).body, "ok");
    assert_eq!(route(&get("/nope")), Response { status: 404, body: NOT_FOUND_HTML.into() });
    assert_eq!(route(&get("/hello?lang=en&name=Ada")).body, "<p>Hello, Ada!</p>");
    assert_eq!(route(&get("/hello")).body, "<p>Hello, stranger!</p>");
    let post = Request { method: "POST".into(), ..get("/") };
    assert_eq!(route(&post), Response { status: 405, body: METHOD_NOT_ALLOWED_HTML.into() });
}

fn serve_once(raw: &'static [u8]) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        handle_connection(stream);
    });
    let mut client = TcpStream::connect(addr).unwrap();
    client.write_all(raw).unwrap();
    let mut out = String::new();
    client.read_to_string(&mut out).unwrap();
    server.join().unwrap();
    out
}

#[test]
fn end_to_end() {
    let ok = serve_once(b"GET /health HTTP/1.1\r\nHost: x\r\n\r\n");
    assert_eq!(ok, Response { status: 200, body: "ok".into() }.to_http());
    let bad = serve_once(b"NONSENSE\r\n\r\n");
    assert_eq!(bad, Response { status: 400, body: BAD_REQUEST_HTML.into() }.to_http());
}
