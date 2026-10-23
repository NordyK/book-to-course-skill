use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};

pub fn build_request(method: &str, host: &str, path: &str, body: &str) -> String {
    let mut req = format!("{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n");
    if !body.is_empty() {
        req.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }
    req.push_str("\r\n");
    req.push_str(body);
    req
}

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub reason: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

pub fn parse_response(raw: &str) -> Result<Response, String> {
    let (head, body) = raw.split_once("\r\n\r\n").ok_or("no blank line after the headers")?;
    let mut lines = head.split("\r\n");
    let status_line = lines.next().ok_or("empty response")?;
    let mut parts = status_line.splitn(3, ' ');
    let version = parts.next().unwrap_or("");
    if !version.starts_with("HTTP/1.") {
        return Err(format!("bad version in {status_line:?}"));
    }
    let status: u16 = parts
        .next()
        .and_then(|c| c.parse().ok())
        .ok_or_else(|| format!("bad status code in {status_line:?}"))?;
    let reason = parts.next().unwrap_or("").to_string();

    let mut headers = Vec::new();
    for line in lines {
        let (name, value) = line.split_once(':').ok_or_else(|| format!("bad header {line:?}"))?;
        headers.push((name.trim().to_string(), value.trim().to_string()));
    }
    let mut response = Response { status, reason, headers, body: String::new() };
    response.body = match response.header("Content-Length").and_then(|v| v.parse::<usize>().ok()) {
        Some(n) => body.get(..n).unwrap_or(body).to_string(),
        None => body.to_string(),
    };
    Ok(response)
}

pub fn fetch(addr: SocketAddr, path: &str) -> io::Result<Response> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(build_request("GET", &addr.to_string(), path, "").as_bytes())?;
    let mut raw = String::new();
    stream.read_to_string(&mut raw)?;
    parse_response(&raw).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests;
