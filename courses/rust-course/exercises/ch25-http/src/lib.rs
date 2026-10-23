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
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

pub fn reason(status: u16) -> &'static str {
    todo!()
}

impl Response {
    pub fn to_http(&self) -> String {
        todo!()
    }
}

pub fn read_request<R: BufRead>(reader: R) -> Option<Request> {
    todo!()
}

pub fn route(req: &Request) -> Response {
    todo!()
}

pub fn handle_connection(stream: TcpStream) {
    todo!()
}

#[cfg(test)]
mod tests;
