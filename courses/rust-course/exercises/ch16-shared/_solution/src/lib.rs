use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn count_words(texts: Vec<String>, threads: usize) -> (BTreeMap<String, usize>, usize) {
    let w = threads.max(1);
    let mut buckets: Vec<Vec<String>> = vec![Vec::new(); w];
    for (i, t) in texts.into_iter().enumerate() {
        buckets[i % w].push(t);
    }

    let shared = Arc::new(Mutex::new(BTreeMap::new()));
    let processed = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for bucket in buckets {
        let shared = Arc::clone(&shared);
        let processed = Arc::clone(&processed);
        handles.push(thread::spawn(move || {
            let mut local: HashMap<String, usize> = HashMap::new();
            for text in &bucket {
                for raw in text.split_whitespace() {
                    let word = raw.trim_matches(|c: char| ".,!?;:".contains(c)).to_lowercase();
                    if !word.is_empty() {
                        *local.entry(word).or_insert(0) += 1;
                    }
                }
                processed.fetch_add(1, Ordering::Relaxed);
            }
            let mut map = shared.lock().unwrap();
            for (k, v) in local {
                *map.entry(k).or_insert(0) += v;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    let map = Arc::try_unwrap(shared).unwrap().into_inner().unwrap();
    (map, processed.load(Ordering::Relaxed))
}

pub struct Stats {
    inner: Mutex<Vec<u64>>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats { inner: Mutex::new(Vec::new()) }
    }

    pub fn record(&self, v: u64) {
        self.inner.lock().unwrap().push(v);
    }

    pub fn mean(&self) -> Option<f64> {
        let v = self.inner.lock().unwrap();
        if v.is_empty() {
            None
        } else {
            Some(v.iter().sum::<u64>() as f64 / v.len() as f64)
        }
    }
}

#[cfg(test)]
mod tests;
