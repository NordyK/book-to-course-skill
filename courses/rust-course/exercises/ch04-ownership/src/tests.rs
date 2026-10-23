use super::*;

#[test]
fn shouting() {
    assert_eq!(shout(String::from("hello")), "HELLO!", "shout(\"hello\")");
    assert_eq!(shout(String::new()), "!", "shout of an empty String");
}

#[test]
fn joining() {
    assert_eq!(join_words(String::from("ground"), String::from("station")), "ground station");
    assert_eq!(join_words(String::from("a"), String::new()), "a ", "an empty second word still gets the space");
}

#[test]
fn two_independent_copies() {
    let (mut a, b) = twice("sat");
    assert_eq!((a.as_str(), b.as_str()), ("sat", "sat"), "twice(\"sat\")");
    a.push('!');
    assert_eq!(b, "sat", "changing the first must not change the second");
}
