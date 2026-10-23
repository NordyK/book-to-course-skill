use super::*;

#[test]
fn splits_words() {
    assert_eq!(words("  a  b\tc\n"), vec!["a", "b", "c"]);
}

#[test]
fn fitting() {
    assert!(fits("", "hello", 5));
    assert!(!fits("", "hello", 4));
    assert!(fits("the", "quick", 9), "3 + 1 + 5 = 9");
    assert!(!fits("the", "quick", 8));
    assert!(fits("", "żółw", 4), "counts characters, not bytes");
}

#[test]
fn wraps_lines() {
    assert_eq!(wrap("the quick brown fox", 10), vec!["the quick", "brown fox"]);
    assert_eq!(wrap("the quick brown fox", 100), vec!["the quick brown fox"]);
}

#[test]
fn long_words_stay_whole() {
    assert_eq!(wrap("a extraordinarily b", 5), vec!["a", "extraordinarily", "b"]);
}

#[test]
fn edge_cases() {
    assert!(wrap("", 10).is_empty());
    assert!(wrap("   ", 10).is_empty());
    assert_eq!(wrap("one   two", 3), vec!["one", "two"], "whitespace collapses");
}
