use super::*;

#[test]
fn good_names() {
    assert_eq!(Username::parse("  ferris_99 ").unwrap().as_str(), "ferris_99");
    assert_eq!(Username::parse("abc").unwrap().as_str(), "abc", "exactly 3");
    assert!(Username::parse("a234567890123456").is_ok(), "exactly 16");
}

#[test]
fn bad_names() {
    assert_eq!(Username::parse("ab"), Err(String::from("too short")));
    assert_eq!(Username::parse("a2345678901234567"), Err(String::from("too long")));
    assert_eq!(Username::parse("bad-name"), Err(String::from("invalid character: -")));
    assert_eq!(Username::parse("9lives"), Err(String::from("starts with a digit")));
    assert_eq!(Username::parse("  x  "), Err(String::from("too short")), "trimmed first");
}

#[test]
fn guesses_in_range() {
    assert_eq!(Guess::new(1).value(), 1);
    assert_eq!(Guess::new(100).value(), 100);
}

#[test]
#[should_panic(expected = "between 1 and 100")]
fn guess_too_big() {
    Guess::new(101);
}

#[test]
#[should_panic(expected = "between 1 and 100")]
fn guess_too_small() {
    Guess::new(0);
}
