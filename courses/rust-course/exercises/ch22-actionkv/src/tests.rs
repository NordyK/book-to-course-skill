use super::*;
use std::io::Cursor;

fn mem() -> ActionKV<Cursor<Vec<u8>>> {
    ActionKV::open(Cursor::new(Vec::new())).unwrap()
}

fn reopen(kv: ActionKV<Cursor<Vec<u8>>>) -> ActionKV<Cursor<Vec<u8>>> {
    let bytes = kv.into_inner().into_inner();
    ActionKV::open(Cursor::new(bytes)).unwrap()
}

#[test]
fn basic_operations() {
    let mut kv = mem();
    assert_eq!(kv.get(b"a").unwrap(), None);
    kv.insert(b"a", b"1").unwrap();
    kv.insert(b"b", b"2").unwrap();
    kv.update(b"a", b"3").unwrap();
    assert_eq!(kv.get(b"a").unwrap(), Some(b"3".to_vec()));
    assert_eq!(kv.get(b"b").unwrap(), Some(b"2".to_vec()));
    kv.delete(b"b").unwrap();
    assert_eq!(kv.get(b"b").unwrap(), None);
    assert_eq!(kv.len(), 1);
}

#[test]
fn record_layout() {
    let mut kv = mem();
    kv.insert(b"key", b"value").unwrap();
    let bytes = kv.into_inner().into_inner();
    assert_eq!(bytes.len(), 12 + 3 + 5);
    assert_eq!(&bytes[0..4], &crc32(b"keyvalue").to_le_bytes());
    assert_eq!(&bytes[4..8], &3u32.to_le_bytes());
    assert_eq!(&bytes[8..12], &5u32.to_le_bytes());
    assert_eq!(&bytes[12..], b"keyvalue");
}

#[test]
fn survives_reopen() {
    let mut kv = mem();
    kv.insert(b"Fiji", b"Suva").unwrap();
    kv.insert(b"Kiribati", b"South Tarawa").unwrap();
    kv.update(b"Kiribati", b"Tarawa").unwrap();
    kv.insert(b"Cook Islands", b"Avarua").unwrap();
    kv.delete(b"Cook Islands").unwrap();
    let mut kv = reopen(kv);
    assert_eq!(kv.get(b"Fiji").unwrap(), Some(b"Suva".to_vec()));
    assert_eq!(kv.get(b"Kiribati").unwrap(), Some(b"Tarawa".to_vec()));
    assert_eq!(kv.get(b"Cook Islands").unwrap(), None);
    assert_eq!(kv.len(), 2);
    kv.insert(b"Tonga", b"Nuku'alofa").unwrap();
    assert_eq!(kv.get(b"Tonga").unwrap(), Some(b"Nuku'alofa".to_vec()));
}

#[test]
fn delete_missing_writes_nothing() {
    let mut kv = mem();
    kv.delete(b"ghost").unwrap();
    assert!(kv.into_inner().into_inner().is_empty());
}

#[test]
fn torn_write_is_ignored() {
    let mut kv = mem();
    kv.insert(b"a", b"1").unwrap();
    kv.insert(b"b", b"2").unwrap();
    let mut bytes = kv.into_inner().into_inner();
    bytes.truncate(bytes.len() - 1); // crash in the middle of the last record
    let mut kv = ActionKV::open(Cursor::new(bytes)).unwrap();
    assert_eq!(kv.get(b"a").unwrap(), Some(b"1".to_vec()));
    assert_eq!(kv.get(b"b").unwrap(), None);
}

#[test]
fn corruption_is_an_error() {
    let mut kv = mem();
    kv.insert(b"a", b"hello").unwrap();
    let mut bytes = kv.into_inner().into_inner();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xff;
    let err = ActionKV::open(Cursor::new(bytes)).err().expect("must fail");
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);
}

#[test]
fn compaction() {
    let mut kv = mem();
    for i in 0..10u8 {
        kv.insert(b"counter", &[b'0' + i]).unwrap();
    }
    kv.insert(b"z", b"last").unwrap();
    kv.insert(b"gone", b"x").unwrap();
    kv.delete(b"gone").unwrap();
    let before = kv.into_inner().into_inner();
    let mut kv = ActionKV::open(Cursor::new(before.clone())).unwrap();

    let mut small = kv.compact(Cursor::new(Vec::new())).unwrap();
    assert_eq!(small.get(b"counter").unwrap(), Some(b"9".to_vec()));
    assert_eq!(small.get(b"z").unwrap(), Some(b"last".to_vec()));
    assert_eq!(small.get(b"gone").unwrap(), None);
    assert_eq!(small.len(), 2);
    let after = small.into_inner().into_inner();
    assert_eq!(after.len(), (12 + 7 + 1) + (12 + 1 + 4));
    assert!(after.len() < before.len());
    assert_eq!(&after[12..19], b"counter", "sorted key order");
}
