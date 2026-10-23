use super::*;

#[test]
fn endpoints() {
    let a = parse_endpoint("127.0.0.1:8080").unwrap();
    assert_eq!(a.port(), 8080);
    assert!(matches!(parse_endpoint("300.0.0.1:80"), Err(NetError::Addr(_))));
    let e = parse_endpoint("10.0.0.1:x").unwrap_err();
    assert!(matches!(e, NetError::Port(_)));
    assert_eq!(e.to_string(), "port: invalid digit found in string");
    assert!(e.source().is_some());
    assert!(matches!(parse_endpoint("no-port"), Err(NetError::Protocol(_))));
}

#[test]
fn io_errors_convert() {
    fn open() -> Result<(), NetError> {
        std::fs::File::open("/definitely/missing")?;
        Ok(())
    }
    let e = open().unwrap_err();
    assert!(matches!(e, NetError::Io(_)));
    assert!(e.to_string().starts_with("io: "));
    let p = NetError::Protocol(String::from("x"));
    assert_eq!(p.to_string(), "protocol: x");
    assert!(p.source().is_none());
}

#[test]
fn macs() {
    let m = Mac::parse("AA-bb-01-02-03-04").unwrap();
    assert_eq!(m.to_string(), "aa:bb:01:02:03:04");
    assert!(m.is_local());
    assert!(!m.is_multicast());
    assert!(Mac::parse("01:00:5e:00:00:fb").unwrap().is_multicast());
    for bad in ["aa:bb:cc", "aa:bb:cc:dd:ee:gg", "aabbccddeeff", "a:bb:cc:dd:ee:ff0"] {
        assert!(matches!(Mac::parse(bad), Err(NetError::Protocol(_))), "{bad}");
    }
    let g = Mac::local_unicast([0xff; 6]);
    assert!(g.is_local() && !g.is_multicast());
    assert_eq!(g.0[0], 0xfe);
    assert_eq!(&g.0[1..], &[0xff; 5]);
}

#[test]
fn happy_path() {
    let mut s = Conn::Closed;
    for e in [Event::Open, Event::SynAck, Event::Send(10), Event::Recv(200), Event::Send(5)] {
        s = step(s, e).unwrap();
    }
    assert_eq!(s, Conn::Established { sent: 15, received: 200 });
    let s = step(s, Event::Close).unwrap();
    assert_eq!(s, Conn::FinWait);
    assert_eq!(step(s, Event::Ack).unwrap(), Conn::Closed);
}

#[test]
fn illegal_transitions() {
    let e = step(Conn::Closed, Event::Send(5)).unwrap_err();
    assert_eq!(e.to_string(), "protocol: Closed cannot handle Send(5)");
    assert!(step(Conn::SynSent, Event::Close).is_err());
    assert!(step(Conn::FinWait, Event::Open).is_err());
    let e = step(Conn::Established { sent: 1, received: 2 }, Event::Open).unwrap_err();
    assert_eq!(e.to_string(), "protocol: Established cannot handle Open");
}
