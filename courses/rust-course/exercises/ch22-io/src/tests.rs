use super::*;
use std::io::Cursor;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("ch22-io-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&p);
    p
}

#[test]
fn grepping() {
    let mut out = Vec::new();
    let n = grep("apple\nbanana\ncherry\nmango".as_bytes(), "an", &mut out).unwrap();
    assert_eq!(n, 2);
    assert_eq!(String::from_utf8(out).unwrap(), "2:banana\n4:mango\n");
    let mut none = Vec::new();
    assert_eq!(grep(Cursor::new(""), "x", &mut none).unwrap(), 0);
    assert!(none.is_empty());
}

#[test]
fn config() {
    let text = "# settings\nname = ferris\n\n  port=8080  \nmotto = a = b\n";
    let cfg = parse_config(text.as_bytes()).unwrap();
    assert_eq!(cfg["name"], "ferris");
    assert_eq!(cfg["port"], "8080");
    assert_eq!(cfg["motto"], "a = b", "split at the first =");
    assert_eq!(cfg.len(), 3);
}

#[test]
fn config_errors() {
    let err = parse_config("a = 1\n\njust words\n".as_bytes()).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    assert!(err.to_string().contains("line 3"), "{err}");
}

#[test]
fn files_roundtrip() {
    let dir = scratch("roundtrip");
    let path = dir.join("nested/deeper/notes.txt");
    save_lines(&path, &["first", "second", ""]).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "first\nsecond\n\n");
    assert_eq!(load_lines(&path).unwrap(), ["first", "second", ""]);
    assert!(load_lines(&dir.join("missing.txt")).unwrap().is_empty());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn biggest() {
    let dir = scratch("biggest");
    fs::create_dir_all(dir.join("sub")).unwrap();
    assert_eq!(largest_file(&dir).unwrap(), None);
    fs::write(dir.join("a.txt"), "12345").unwrap();
    fs::write(dir.join("b.txt"), "123456789").unwrap();
    fs::write(dir.join("sub/huge.txt"), "x".repeat(1000)).unwrap();
    assert_eq!(largest_file(&dir).unwrap(), Some((String::from("b.txt"), 9)));
    fs::remove_dir_all(&dir).unwrap();
}
