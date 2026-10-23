use super::*;

#[test]
fn plain_number_with_newline() {
    assert_eq!(parse_guess("42\n"), Some(42), "input \"42\\n\"");
}

#[test]
fn spaces_and_windows_newline_are_ignored() {
    assert_eq!(parse_guess("  7  "), Some(7), "input \"  7  \"");
    assert_eq!(parse_guess("15\r\n"), Some(15), "input \"15\\r\\n\"");
}

#[test]
fn text_is_not_a_guess() {
    assert_eq!(parse_guess("foo\n"), None, "input \"foo\\n\"");
    assert_eq!(parse_guess("\n"), None, "an empty line");
    assert_eq!(parse_guess("4 2"), None, "input \"4 2\"");
}

#[test]
fn numbers_outside_1_to_100_are_rejected() {
    assert_eq!(parse_guess("0"), None, "input \"0\"");
    assert_eq!(parse_guess("101"), None, "input \"101\"");
    assert_eq!(parse_guess("-5"), None, "input \"-5\"");
}

#[test]
fn both_ends_of_the_range_are_allowed() {
    assert_eq!(parse_guess("1"), Some(1), "input \"1\"");
    assert_eq!(parse_guess("100"), Some(100), "input \"100\"");
}
