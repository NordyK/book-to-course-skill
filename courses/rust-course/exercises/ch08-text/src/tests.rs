use super::*;

#[test]
fn counting_chars() {
    assert_eq!(char_count("héllo"), 5);
    assert_eq!(char_count("Здравствуйте"), 12);
    assert_eq!(char_count(""), 0);
}

#[test]
fn pig() {
    assert_eq!(pig_latin("apple"), "apple-hay");
    assert_eq!(pig_latin("first"), "irst-fay");
    assert_eq!(pig_latin("e"), "e-hay");
    assert_eq!(pig_latin("x"), "-xay");
}

#[test]
fn truncating() {
    assert_eq!(truncate_chars("hello", 10), "hello", "short text is unchanged");
    assert_eq!(truncate_chars("hello", 5), "hello", "exactly max");
    assert_eq!(truncate_chars("hello world", 6), "hello…");
    assert_eq!(truncate_chars("Здравствуйте", 5), "Здра…", "never cuts inside a character");
}

#[test]
fn words() {
    assert_eq!(word_count("  the quick\tbrown\nfox "), 4);
    assert_eq!(word_count(""), 0);
}
