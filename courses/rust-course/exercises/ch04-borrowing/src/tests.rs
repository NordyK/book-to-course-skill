use super::*;

#[test]
fn vowels() {
    assert_eq!(count_vowels(&String::from("Rust in Action")), 5, "Rust in Action: u, i, A, i, o");
    assert_eq!(count_vowels(&String::from("rhythm")), 0, "rhythm");
    assert_eq!(count_vowels(&String::from("AEIOU aeiou")), 10, "all vowels");
}

#[test]
fn vowels_do_not_take_ownership() {
    let s = String::from("borrowed");
    let _ = count_vowels(&s);
    assert_eq!(s, "borrowed", "the caller still owns s");
}

#[test]
fn title() {
    let mut s = String::from("ferris");
    make_title(&mut s);
    assert_eq!(s, "Ferris", "make_title(\"ferris\")");
    let mut already = String::from("Crab");
    make_title(&mut already);
    assert_eq!(already, "Crab", "already uppercase");
}

#[test]
fn title_of_empty() {
    let mut s = String::new();
    make_title(&mut s);
    assert_eq!(s, "", "empty stays empty");
}

#[test]
fn points() {
    let mut total = 10;
    add_points(&mut total, &[1, 2, 3]);
    assert_eq!(total, 16, "10 + 1 + 2 + 3");
    add_points(&mut total, &[0, 0, 4]);
    assert_eq!(total, 20, "a second call keeps adding");
}

#[test]
fn longest() {
    let a = String::from("orbit");
    let b = String::from("constellation");
    assert_eq!(longer(&a, &b), "constellation");
    let c = String::from("12345");
    assert!(std::ptr::eq(longer(&a, &c), &a), "a tie returns the first argument");
}
