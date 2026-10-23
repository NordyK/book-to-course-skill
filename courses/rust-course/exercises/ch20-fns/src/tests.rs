use super::*;

fn triple(x: i64) -> i64 {
    x * 3
}

#[test]
fn fn_pointers() {
    assert_eq!(apply_n(triple, 3, 1), 27);
    assert_eq!(apply_n(|x| x - 1, 0, 9), 9);
    assert_eq!(apply_n(i64::abs, 1, -4), 4);
}

#[test]
fn closures() {
    let add5 = make_adder(5);
    assert_eq!(add5(10), 15);
    let mut c = make_counter();
    assert_eq!((c(), c(), c()), (1, 2, 3));
    let mut d = make_counter();
    assert_eq!(d(), 1, "each counter is independent");
}

#[test]
fn pipelines() {
    assert_eq!(pipeline(&["inc", "double"])(5), 12);
    assert_eq!(pipeline(&["double", "inc"])(5), 11);
    assert_eq!(pipeline(&["neg", "bogus", "inc"])(5), -4);
    assert_eq!(pipeline(&[])(7), 7);
}

#[test]
fn unsized_types() {
    assert_eq!(longest_len::<str>(&["a", "abc", "ab"]), 3);
    let owned = [String::from("hello"), String::from("hi")];
    assert_eq!(longest_len(&[&owned[0], &owned[1]]), 5);
    assert_eq!(longest_len::<str>(&[]), 0);
}

#[test]
fn never() {
    assert_eq!(parse_or_exit("42"), 42);
}

#[test]
#[should_panic(expected = "bad number: x")]
fn never_panics() {
    parse_or_exit("x");
}
