use super::*;

#[test]
fn even_numbers() {
    assert_eq!(evens(&[1, 2, 3, 4]), vec![2, 4]);
    assert_eq!(evens(&[-2, 0, 7]), vec![-2, 0], "zero and negatives");
    assert_eq!(evens(&[]), Vec::<i32>::new());
}

#[test]
fn totals() {
    assert_eq!(running_total(&[1, 2, 3]), vec![1, 3, 6]);
    assert_eq!(running_total(&[5, -5, 5]), vec![5, 0, 5]);
    assert_eq!(running_total(&[]), Vec::<i32>::new());
}

#[test]
fn duplicates() {
    let mut v = vec![3, 1, 3, 2, 1];
    remove_duplicates(&mut v);
    assert_eq!(v, vec![3, 1, 2]);
    let mut same = vec![7, 7, 7];
    remove_duplicates(&mut same);
    assert_eq!(same, vec![7]);
}

#[test]
fn medians() {
    assert_eq!(median(&[5.0, 1.0, 3.0]), Some(3.0), "odd count");
    assert_eq!(median(&[5.0, 1.0, 3.0, 2.0]), Some(2.5), "even count");
    assert_eq!(median(&[]), None);
}

#[test]
fn median_does_not_reorder_input() {
    let data = [3.0, 1.0, 2.0];
    let _ = median(&data);
    assert_eq!(data, [3.0, 1.0, 2.0]);
}
