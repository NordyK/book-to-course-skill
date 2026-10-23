use super::*;

#[test]
fn words() {
    assert_eq!(longest_word("a quick brown fox"), "quick");
    assert_eq!(longest_word("   "), "");
    assert_eq!(longest_word("ab cd"), "ab", "first on a tie");
}

#[test]
fn longer() {
    assert_eq!(longer_of("pear", "banana"), "banana");
    assert_eq!(longer_of("abc", "xyz"), "abc");
}

#[test]
fn prefix_is_not_tied_to_result() {
    let text = String::from("key=value");
    let result;
    {
        let prefix = String::from("key=");
        result = after_prefix(&text, &prefix);
    }
    assert_eq!(result, Some("value"));
    assert_eq!(after_prefix("abc", "x"), None);
}

#[test]
fn tokens() {
    let mut t = Tokens::new(" a, b ,,c");
    assert_eq!(t.next_token(), Some("a"));
    assert_eq!(t.next_token(), Some("b"));
    assert_eq!(t.next_token(), Some("c"), "empty pieces are skipped");
    assert_eq!(t.next_token(), None);
    assert_eq!(t.next_token(), None);
}

#[test]
fn tokens_outlive_the_tokenizer_borrow() {
    let text = String::from("x,y");
    let mut t = Tokens::new(&text);
    let first = t.next_token();
    let second = t.next_token();
    assert_eq!((first, second), (Some("x"), Some("y")));
}
