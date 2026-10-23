use super::*;

#[test]
fn composing() {
    let h = compose(|x: i32| x + 1, |x: i32| x * 10);
    assert_eq!(h(4), 50);
    let len_then_double = compose(|s: &str| s.len(), |n: usize| n * 2);
    assert_eq!(len_then_double("abc"), 6);
}

#[test]
fn counting() {
    let mut c = counter();
    assert_eq!((c(), c(), c()), (1, 2, 3));
    let mut other = counter();
    assert_eq!(other(), 1, "each counter has its own state");
}

#[test]
fn retrying() {
    let mut calls = 0;
    let r: Result<u32, String> = retry(5, || {
        calls += 1;
        if calls < 3 { Err(format!("fail {calls}")) } else { Ok(calls) }
    });
    assert_eq!(r, Ok(3));
    assert_eq!(calls, 3, "stops at the first Ok");

    let mut tries = 0;
    let r: Result<(), String> = retry(2, || {
        tries += 1;
        Err(format!("fail {tries}"))
    });
    assert_eq!(r, Err(String::from("fail 2")), "the last error");

    let mut once = 0;
    let _ = retry(0, || -> Result<(), ()> { once += 1; Err(()) });
    assert_eq!(once, 1, "at least once");
}

#[test]
fn grouping() {
    assert_eq!(group_by_key(vec![1, 3, 2, 4, 5], |x| x % 2), vec![(1, vec![1, 3]), (0, vec![2, 4]), (1, vec![5])]);
    let words = vec!["apple", "avocado", "banana", "blueberry", "cherry"];
    let g = group_by_key(words, |w| w.chars().next().unwrap());
    assert_eq!(g, vec![('a', vec!["apple", "avocado"]), ('b', vec!["banana", "blueberry"]), ('c', vec!["cherry"])]);
    let empty: Vec<(i32, Vec<i32>)> = group_by_key(Vec::<i32>::new(), |x| *x);
    assert!(empty.is_empty());
}
