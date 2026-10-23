use super::*;

#[test]
fn lengths() {
    assert_eq!(c_len(c""), 0);
    assert_eq!(c_len(c"hello"), 5);
    let s = CString::new("ünï").unwrap();
    assert_eq!(c_len(&s), 5, "strlen counts bytes, not characters");
}

#[test]
fn division() {
    assert_eq!(c_div(17, 5), Some((3, 2)));
    assert_eq!(c_div(-17, 5), Some((-3, -2)));
    assert_eq!(c_div(1, 0), None);
    assert_eq!(c_div(i32::MIN, -1), None);
    assert_eq!(c_div(i32::MIN, 1), Some((i32::MIN, 0)));
}

#[test]
fn parsing() {
    assert_eq!(parse_long("42", 10), Some((42, 2)));
    assert_eq!(parse_long("  -17xyz", 10), Some((-17, 5)));
    assert_eq!(parse_long("ff", 16), Some((255, 2)));
    assert_eq!(parse_long("0x1F rest", 0), Some((31, 4)));
    assert_eq!(parse_long("1011", 2), Some((11, 4)));
    assert_eq!(parse_long("xyz", 10), None);
    assert_eq!(parse_long("", 10), None);
    assert_eq!(parse_long("12\0 3", 10), None);
    assert_eq!(parse_long("12", 1), None);
    assert_eq!(parse_long("12", 37), None);
}

#[test]
fn sorting() {
    let mut v = [3, -1, 7, 0, 7, -5];
    sort_desc(&mut v);
    assert_eq!(v, [7, 7, 3, 0, -1, -5]);
    let mut empty: [i32; 0] = [];
    sort_desc(&mut empty);
    let mut extremes = [i32::MIN, i32::MAX, 0, -1, 1];
    sort_desc(&mut extremes);
    assert_eq!(extremes, [i32::MAX, 1, 0, -1, i32::MIN]);
}

#[test]
fn padding() {
    assert_eq!(zero_pad(42, 5), "00042");
    assert_eq!(zero_pad(-42, 5), "-0042");
    assert_eq!(zero_pad(123_456, 3), "123456");
    assert_eq!(zero_pad(7, 0), "7");
    assert_eq!(zero_pad(i32::MIN, 12), "-02147483648");
}
