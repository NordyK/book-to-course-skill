use super::*;

#[test]
fn sums() {
    let data: Vec<u64> = (1..=1000).collect();
    assert_eq!(parallel_sum(&data, 4), 500500);
    assert_eq!(parallel_sum(&data, 7), 500500, "uneven chunks");
    assert_eq!(parallel_sum(&[1, 2, 3, 4, 5], 10), 15, "more parts than items");
    assert_eq!(parallel_sum(&[], 3), 0);
    assert_eq!(parallel_sum(&[5], 0), 5, "zero parts treated as one");
}

#[test]
fn maps_in_order() {
    let out = map_in_threads(vec!["a".into(), "bc".into(), "def".into()], |s| s.to_uppercase());
    assert_eq!(out, vec!["A", "BC", "DEF"]);
    let lens = map_in_threads(vec!["xyz".into()], |s| s.len().to_string());
    assert_eq!(lens, vec!["3"]);
}

#[test]
fn catches_panics() {
    assert_eq!(run_safely(|| 42), Ok(42));
    assert_eq!(run_safely(|| panic!("boom")), Err(String::from("job panicked")));
}
