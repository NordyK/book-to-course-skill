use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub const HELLO_HTML: &str = "<html><body><h1>Hello!</h1></body></html>";
pub const NOT_FOUND_HTML: &str = "<html><body><h1>Oops!</h1></body></html>";
pub const BAD_REQUEST_HTML: &str = "<html><body><h1>Bad request</h1></body></html>";
pub const METHOD_NOT_ALLOWED_HTML: &str = "<html><body><h1>Method not allowed</h1></body></html>";

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

pub fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "BAD REQUEST",
        404 => "NOT FOUND",
        405 => "METHOD NOT ALLOWED",
        _ => "",
    }
}

impl Response {
    fn new(status: u16, body: impl Into<String>) -> Response {
        Response { status, body: body.into() }
    }

    pub fn to_http(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
            self.status,
            reason(self.status),
            self.body.len(),
            self.body
        )
    }
}

pub fn read_request<R: BufRead>(reader: R) -> Option<Request> {
    let mut lines = reader.lines().map_while(Result::ok);
    let first = lines.next()?;
    let parts: Vec<&str> = first.trim_end_matches('\r').split_whitespace().collect();
    if parts.len() != 3 || !parts[2].starts_with("HTTP/") {
        return None;
    }
    let mut headers = Vec::new();
    for line in lines {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            headers.push((name.trim().to_string(), value.trim().to_string()));
        }
    }
    Some(Request { method: parts[0].to_string(), path: parts[1].to_string(), headers })
}

pub fn route(req: &Request) -> Response {
    if req.method != "GET" {
        return Response::new(405, METHOD_NOT_ALLOWED_HTML);
    }
    let (path, query) = req.path.split_once('?').unwrap_or((&req.path, ""));
    match path {
        "/" => Response::new(200, HELLO_HTML),
        "/health" => Response::new(200, "ok"),
        "/hello" => {
            let name = query.split('&').find_map(|kv| kv.strip_prefix("name=")).unwrap_or("stranger");
            Response::new(200, format!("<p>Hello, {name}!</p>"))
        }
        _ => Response::new(404, NOT_FOUND_HTML),
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let response = match read_request(BufReader::new(&stream)) {
        Some(req) => route(&req),
        None => Response::new(400, BAD_REQUEST_HTML),
    };
    // A client that already hung up is not our problem; ignore the write error.
    let _ = stream.write_all(response.to_http().as_bytes());
}

#[cfg(test)]
mod tests;
