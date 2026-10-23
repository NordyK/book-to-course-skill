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

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetError::Io(e) => write!(f, "io: {e}"),
            NetError::Addr(e) => write!(f, "address: {e}"),
            NetError::Port(e) => write!(f, "port: {e}"),
            NetError::Protocol(m) => write!(f, "protocol: {m}"),
        }
    }
}

impl Error for NetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NetError::Io(e) => Some(e),
            NetError::Addr(e) => Some(e),
            NetError::Port(e) => Some(e),
            NetError::Protocol(_) => None,
        }
    }
}

impl From<io::Error> for NetError {
    fn from(e: io::Error) -> Self {
        NetError::Io(e)
    }
}

impl From<AddrParseError> for NetError {
    fn from(e: AddrParseError) -> Self {
        NetError::Addr(e)
    }
}

impl From<ParseIntError> for NetError {
    fn from(e: ParseIntError) -> Self {
        NetError::Port(e)
    }
}

pub fn parse_endpoint(s: &str) -> Result<SocketAddr, NetError> {
    let (ip, port) = s
        .rsplit_once(':')
        .ok_or_else(|| NetError::Protocol(format!("{s:?} has no port")))?;
    let ip: Ipv4Addr = ip.parse()?;
    let port: u16 = port.parse()?;
    Ok(SocketAddr::from((ip, port)))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mac(pub [u8; 6]);

impl Mac {
    pub fn parse(s: &str) -> Result<Mac, NetError> {
        let bad = || NetError::Protocol(format!("{s:?} is not a MAC address"));
        let parts: Vec<&str> = s.split(|c| c == ':' || c == '-').collect();
        if parts.len() != 6 || parts.iter().any(|p| p.len() != 2) {
            return Err(bad());
        }
        let mut bytes = [0u8; 6];
        for (b, p) in bytes.iter_mut().zip(&parts) {
            *b = u8::from_str_radix(p, 16).map_err(|_| bad())?;
        }
        Ok(Mac(bytes))
    }
    pub fn is_local(&self) -> bool {
        self.0[0] & 0b10 != 0
    }
    pub fn is_multicast(&self) -> bool {
        self.0[0] & 0b01 != 0
    }
    pub fn local_unicast(mut bytes: [u8; 6]) -> Mac {
        bytes[0] = (bytes[0] | 0b10) & !0b01;
        Mac(bytes)
    }
}

impl fmt::Display for Mac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self.0.iter().map(|b| format!("{b:02x}")).collect();
        write!(f, "{}", parts.join(":"))
    }
}

#[derive(Debug, PartialEq)]
pub enum Conn {
    Closed,
    SynSent,
    Established { sent: usize, received: usize },
    FinWait,
}

impl Conn {
    fn name(&self) -> &'static str {
        match self {
            Conn::Closed => "Closed",
            Conn::SynSent => "SynSent",
            Conn::Established { .. } => "Established",
            Conn::FinWait => "FinWait",
        }
    }
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
    match (state, event) {
        (Conn::Closed, Event::Open) => Ok(Conn::SynSent),
        (Conn::SynSent, Event::SynAck) => Ok(Conn::Established { sent: 0, received: 0 }),
        (Conn::Established { sent, received }, Event::Send(n)) => Ok(Conn::Established { sent: sent + n, received }),
        (Conn::Established { sent, received }, Event::Recv(n)) => Ok(Conn::Established { sent, received: received + n }),
        (Conn::Established { .. }, Event::Close) => Ok(Conn::FinWait),
        (Conn::FinWait, Event::Ack) => Ok(Conn::Closed),
        (s, e) => Err(NetError::Protocol(format!("{} cannot handle {:?}", s.name(), e))),
    }
}

#[cfg(test)]
mod tests;
