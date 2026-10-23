use super::*;

#[test]
fn first_words() {
    assert_eq!(first_word("ground control"), "ground");
    assert_eq!(first_word("single"), "single", "no space: the whole text");
    assert_eq!(first_word(""), "", "empty text");
    let owned = String::from("major tom");
    assert_eq!(first_word(&owned), "major", "works with a &String too");
}

#[test]
fn nth_words() {
    assert_eq!(nth_word("to be or not", 0), Some("to"));
    assert_eq!(nth_word("to be or not", 2), Some("or"));
    assert_eq!(nth_word("to be or not", 3), Some("not"));
    assert_eq!(nth_word("to be or not", 4), None, "only four words");
}

#[test]
fn largest_values() {
    assert_eq!(largest(&[3, 9, -1]), Some(9));
    assert_eq!(largest(&[-5, -2, -9]), Some(-2), "all negative");
    assert_eq!(largest(&[]), None, "empty slice");
    let a = [1, 50, 2, 99];
    assert_eq!(largest(&a[..3]), Some(50), "only part of an array");
}

#[test]
fn zeroing() {
    let mut v = [4, -2, 0, -7];
    zero_negatives(&mut v);
    assert_eq!(v, [4, 0, 0, 0]);
    let mut w = [-1, -1, -1, -1];
    zero_negatives(&mut w[2..]);
    assert_eq!(w, [-1, -1, 0, 0], "only the slice is changed");
}
