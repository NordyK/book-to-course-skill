use super::*;

#[test]
fn parsing() {
    assert_eq!(parse("add"), Some(Command::Add));
    assert_eq!(parse("dup"), Some(Command::Dup));
    assert_eq!(parse("-42"), Some(Command::Push(-42)));
    assert_eq!(parse("plus"), None, "unknown words are None");
}

#[test]
fn applying() {
    let mut s = vec![2, 3];
    assert_eq!(apply(&mut s, Command::Add), Ok(()));
    assert_eq!(s, vec![5]);
    assert_eq!(apply(&mut s, Command::Dup), Ok(()));
    assert_eq!(s, vec![5, 5]);
    assert_eq!(apply(&mut s, Command::Pop), Ok(()));
    assert_eq!(s, vec![5]);
}

#[test]
fn underflow_leaves_stack_alone() {
    let mut s = vec![7];
    assert_eq!(apply(&mut s, Command::Mul), Err(String::from("stack underflow")));
    assert_eq!(s, vec![7], "the stack is unchanged after an error");
    let mut empty = Vec::new();
    assert_eq!(apply(&mut empty, Command::Pop), Err(String::from("stack underflow")));
    assert_eq!(apply(&mut empty, Command::Dup), Err(String::from("stack underflow")));
}

#[test]
fn programs() {
    assert_eq!(eval("2 3 add 4 mul"), Ok(Some(20)));
    assert_eq!(eval("5 dup mul"), Ok(Some(25)));
    assert_eq!(eval(""), Ok(None), "an empty program leaves an empty stack");
    assert_eq!(eval("1 add"), Err(String::from("stack underflow")));
    assert_eq!(eval("2 x"), Err(String::from("unknown word: x")));
}
