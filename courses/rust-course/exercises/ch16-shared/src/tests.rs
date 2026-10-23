use super::*;

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

#[test]
fn counts_across_threads() {
    let (map, n) = count_words(s(&["Hi, there.", "hi!", "There there"]), 2);
    assert_eq!(n, 3);
    assert_eq!(map.get("hi"), Some(&2));
    assert_eq!(map.get("there"), Some(&3));
    assert_eq!(map.len(), 2);
}

#[test]
fn many_texts_many_threads() {
    let texts: Vec<String> = (0..200).map(|i| format!("word{} common", i % 5)).collect();
    let (map, n) = count_words(texts, 8);
    assert_eq!(n, 200);
    assert_eq!(map["common"], 200);
    assert_eq!(map["word3"], 40);
}

#[test]
fn edge_cases() {
    let (map, n) = count_words(vec![], 4);
    assert!(map.is_empty());
    assert_eq!(n, 0);
    let (map, _) = count_words(s(&["... !!"]), 0);
    assert!(map.is_empty(), "punctuation-only words are skipped");
}

#[test]
fn stats_shared_by_threads() {
    let stats = Stats::new();
    assert_eq!(stats.mean(), None);
    thread::scope(|sc| {
        for t in 0..4u64 {
            let stats = &stats;
            sc.spawn(move || {
                for i in 0..10u64 {
                    stats.record(t * 10 + i);
                }
            });
        }
    });
    assert_eq!(stats.mean(), Some(19.5));
}
