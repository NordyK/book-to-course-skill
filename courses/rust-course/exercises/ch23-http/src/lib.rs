use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};

pub fn build_request(method: &str, host: &str, path: &str, body: &str) -> String {
    todo!()
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
        todo!()
    }
}

pub fn parse_response(raw: &str) -> Result<Response, String> {
    todo!()
}

pub fn fetch(addr: SocketAddr, path: &str) -> io::Result<Response> {
    todo!()
}

#[cfg(test)]
mod tests;
