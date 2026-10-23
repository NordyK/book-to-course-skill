use super::*;

#[test]
fn largest_works_for_many_types() {
    assert_eq!(largest(&[3, 9, 2]), Some(&9));
    assert_eq!(largest(&['q', 'z', 'a']), Some(&'z'));
    assert_eq!(largest(&[0.5, -1.0]), Some(&0.5));
    let empty: [i32; 0] = [];
    assert_eq!(largest(&empty), None, "empty slice");
}

#[test]
fn counting() {
    assert_eq!(count_matching(&["a", "b", "a"], &"a"), 2);
    assert_eq!(count_matching(&[1, 2, 3], &7), 0);
}

#[test]
fn pairs() {
    let p = Pair::new(String::from("x"), String::from("y"));
    assert_eq!(p.swap(), Pair { a: String::from("y"), b: String::from("x") });
    assert_eq!(*Pair::new(4, 7).larger(), 7);
    assert_eq!(*Pair::new("b", "a").larger(), "b");
}

#[test]
fn mean_for_floats() {
    assert_eq!(Pair::new(1.0, 2.0).mean(), 1.5);
}
