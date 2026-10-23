use super::*;

#[test]
fn lengths() {
    assert_eq!(
        word_lengths("Hello  Wörld rust"),
        vec![("hello".to_string(), 5), ("wörld".to_string(), 5), ("rust".to_string(), 4)]
    );
    assert!(word_lengths("   ").is_empty());
}

#[test]
fn top_scores() {
    let s: Vec<(String, u32)> = [("a", 5), ("b", 9), ("c", 7), ("d", 9)].iter().map(|(n, v)| (n.to_string(), *v)).collect();
    assert_eq!(top_n(&s, 3), vec!["b", "d", "c"], "ties keep order");
    assert_eq!(top_n(&s, 10).len(), 4);
    assert!(top_n(&s, 0).is_empty());
}

#[test]
fn totals() {
    assert_eq!(running_total(&[1, 2, 3]), vec![1, 3, 6]);
    assert_eq!(running_total(&[5, -5, 2]), vec![5, 0, 2]);
    assert!(running_total(&[]).is_empty());
}

#[test]
fn parsing() {
    assert_eq!(parse_all(&["1", "-2", "30"]), Ok(vec![1, -2, 30]));
    assert!(parse_all(&["1", "x", "3"]).is_err());
    assert_eq!(parse_all(&[]), Ok(vec![]));
}

#[test]
fn collatz_sequence() {
    assert_eq!(collatz(6).collect::<Vec<_>>(), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    assert_eq!(collatz(1).collect::<Vec<_>>(), vec![1]);
    assert_eq!(collatz(27).count(), 112);
    assert_eq!(collatz(27).max(), Some(9232), "adapters come for free");
}
