use super::*;

#[test]
fn results_in_order() {
    assert_eq!(fan_out(vec![1, 2, 3, 4], 2, |x| x * x), vec![1, 4, 9, 16]);
    let jobs: Vec<u64> = (0..50).collect();
    let expected: Vec<u64> = jobs.iter().map(|x| x + 100).collect();
    assert_eq!(fan_out(jobs, 4, |x| x + 100), expected);
}

#[test]
fn edge_cases() {
    assert!(fan_out(vec![], 3, |x| x).is_empty());
    assert_eq!(fan_out(vec![7], 0, |x| x * 2), vec![14], "zero workers treated as one");
    assert_eq!(fan_out(vec![1, 2], 8, |x| x), vec![1, 2], "more workers than jobs");
}

#[test]
fn smallest_match() {
    assert_eq!(first_match(vec![9, 4, 7, 2], |x| x % 2 == 0), Some(2));
    assert_eq!(first_match(vec![9, 7], |x| x % 2 == 0), None);
    assert_eq!(first_match(vec![], |_| true), None);
}
