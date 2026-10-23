use super::*;
use std::io::Cursor;

#[test]
fn short_dump() {
    let out = hexdump(&b"Hi!\n"[..]).unwrap();
    let expected = format!("[0x00000000] 48 69 21 0a {} |Hi!.|\n", "   ".repeat(12));
    assert_eq!(out, expected);
    assert_eq!(hexdump(&b""[..]).unwrap(), "");
}

#[test]
fn full_and_partial_lines() {
    let data: Vec<u8> = (0u8..20).map(|i| i + 0x41).collect();
    let out = hexdump(Cursor::new(data)).unwrap();
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 2, "the last partial line must not be dropped");
    assert_eq!(lines[0], "[0x00000000] 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f 50  |ABCDEFGHIJKLMNOP|");
    assert!(lines[1].starts_with("[0x00000010] 51 52 53 54 "));
    assert!(lines[1].ends_with(" |QRST|"));
    assert_eq!(lines[0].len(), lines[1].len() + 12, "padding keeps the text column aligned");
}

#[test]
fn non_printable() {
    let out = hexdump(&[0x00u8, 0x7f, 0xff, b'~', b' '][..]).unwrap();
    assert!(out.ends_with("|...~ |\n"), "{out}");
}

#[test]
fn checksum() {
    assert_eq!(crc32(b"123456789"), 0xcbf4_3926);
    assert_eq!(crc32(b""), 0);
    assert_ne!(crc32(b"abc"), crc32(b"abd"));
}

#[test]
fn records_roundtrip() {
    let rec = encode_record(b"name", b"Ferris");
    assert_eq!(rec.len(), 12 + 4 + 6);
    assert_eq!(&rec[4..8], &4u32.to_le_bytes());
    assert_eq!(&rec[8..12], &6u32.to_le_bytes());
    assert_eq!(&rec[0..4], &crc32(b"nameFerris").to_le_bytes());

    let mut stream = rec.clone();
    stream.extend(encode_record(b"", b"x"));
    let mut r = Cursor::new(stream);
    assert_eq!(decode_record(&mut r).unwrap(), (b"name".to_vec(), b"Ferris".to_vec()));
    assert_eq!(decode_record(&mut r).unwrap(), (vec![], b"x".to_vec()));
    assert_eq!(decode_record(&mut r).unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
}

#[test]
fn corruption_detected() {
    let mut rec = encode_record(b"k", b"value");
    let last = rec.len() - 1;
    rec[last] ^= 1;
    let err = decode_record(&mut Cursor::new(rec)).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);
}
