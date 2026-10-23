use super::*;

#[test]
fn good_readings() {
    assert_eq!(parse_reading("temp 21.5"), Ok((String::from("temp"), 21.5)));
    assert_eq!(parse_reading("  hum   40 "), Ok((String::from("hum"), 40.0)), "extra spaces");
}

#[test]
fn error_variants() {
    assert_eq!(parse_reading("   "), Err(ReadingError::Empty));
    assert_eq!(parse_reading("temp"), Err(ReadingError::NoValue(String::from("temp"))));
    assert!(matches!(parse_reading("temp hot"), Err(ReadingError::BadValue(_))), "not a number");
    assert_eq!(parse_reading("temp 200"), Err(ReadingError::OutOfRange(200.0)));
    assert_eq!(parse_reading("temp -50"), Ok((String::from("temp"), -50.0)), "the range is inclusive");
}

#[test]
fn messages() {
    assert_eq!(ReadingError::Empty.to_string(), "empty line");
    assert_eq!(parse_reading("temp").unwrap_err().to_string(), "no value for temp");
    assert_eq!(parse_reading("t x").unwrap_err().to_string(), "bad value: invalid float literal");
    assert_eq!(parse_reading("t 151").unwrap_err().to_string(), "out of range: 151");
}

#[test]
fn question_mark_converts() {
    let e: ReadingError = "x".parse::<f64>().unwrap_err().into();
    assert!(matches!(e, ReadingError::BadValue(_)), "From<ParseFloatError> is implemented");
}

#[test]
fn averages() {
    assert_eq!(average(&["a 10", "b 20"]), Ok(15.0));
    assert_eq!(average(&[]), Err(ReadingError::Empty));
    assert_eq!(average(&["a 10", "b", "c 999"]), Err(ReadingError::NoValue(String::from("b"))), "first error wins");
}
