use std::error::Error;
use std::fmt;
use std::io;
use std::net::{AddrParseError, Ipv4Addr, SocketAddr};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum NetError {
    Io(io::Error),
    Addr(AddrParseError),
    Port(ParseIntError),
    Protocol(String),
}

// TODO: Display, Error, From x3

pub fn parse_endpoint(s: &str) -> Result<SocketAddr, NetError> {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mac(pub [u8; 6]);

impl Mac {
    pub fn parse(s: &str) -> Result<Mac, NetError> {
        todo!()
    }
    pub fn is_local(&self) -> bool {
        todo!()
    }
    pub fn is_multicast(&self) -> bool {
        todo!()
    }
    pub fn local_unicast(bytes: [u8; 6]) -> Mac {
        todo!()
    }
}

// TODO: Display for Mac

#[derive(Debug, PartialEq)]
pub enum Conn {
    Closed,
    SynSent,
    Established { sent: usize, received: usize },
    FinWait,
}

#[derive(Debug)]
pub enum Event {
    Open,
    SynAck,
    Send(usize),
    Recv(usize),
    Close,
    Ack,
}

pub fn step(state: Conn, event: Event) -> Result<Conn, NetError> {
    todo!()
}

#[cfg(test)]
mod tests;
