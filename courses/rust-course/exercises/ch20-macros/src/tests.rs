use super::*;
use std::collections::HashMap;

newtype!(Meters, f64);
newtype!(UserId, u32);

#[test]
fn maps() {
    let m: HashMap<&str, i32> = hashmap! { "one" => 1, "two" => 2, };
    assert_eq!(m.len(), 2);
    assert_eq!(m["two"], 2);
    let e: HashMap<i32, i32> = hashmap! {};
    assert!(e.is_empty());
}

#[test]
fn minimum() {
    assert_eq!(min!(7), 7);
    assert_eq!(min!(4, 2, 8), 2);
    assert_eq!(min!(3.5, -1.0), -1.0);
    let mut calls = 0;
    let mut next = || {
        calls += 1;
        calls
    };
    assert_eq!(min!(next(), 5), 1);
    assert_eq!(calls, 1, "each argument is evaluated once");
}

#[test]
fn counting() {
    const N: usize = count!(a, b, c);
    assert_eq!(N, 3);
    assert_eq!(count!(), 0);
    assert_eq!(count!(x), 1);
    assert_eq!(count!((1, 2), [3], "four"), 3);
}

#[test]
fn newtypes() {
    let m = Meters::from(2.5);
    assert_eq!(m, Meters(2.5));
    assert_eq!(m.to_string(), "2.5");
    let copy = m;
    assert_eq!(format!("{copy:?}"), "Meters(2.5)");
    assert_eq!(UserId::from(7).0, 7);
}

#[test]
fn all_true() {
    assert_all!(1 + 1 == 2, "a" < "b", true);
}

#[test]
#[should_panic(expected = "assertion failed: 2 > 3")]
fn one_false() {
    assert_all!(1 < 2, 2 > 3, false);
}
