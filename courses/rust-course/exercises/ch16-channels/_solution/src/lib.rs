use std::sync::mpsc;
use std::thread;

pub fn fan_out(jobs: Vec<u64>, workers: usize, f: fn(u64) -> u64) -> Vec<u64> {
    let w = workers.max(1);
    let mut buckets: Vec<Vec<(usize, u64)>> = vec![Vec::new(); w];
    for (i, job) in jobs.into_iter().enumerate() {
        buckets[i % w].push((i, job));
    }

    let (tx, rx) = mpsc::channel();
    for bucket in buckets {
        let tx = tx.clone();
        thread::spawn(move || {
            for (i, job) in bucket {
                tx.send((i, f(job))).unwrap();
            }
        });
    }
    drop(tx);

    let mut results: Vec<(usize, u64)> = rx.iter().collect();
    results.sort_by_key(|(i, _)| *i);
    results.into_iter().map(|(_, v)| v).collect()
}

pub fn first_match(candidates: Vec<u64>, test: fn(u64) -> bool) -> Option<u64> {
    let (tx, rx) = mpsc::channel();
    for c in candidates {
        let tx = tx.clone();
        thread::spawn(move || {
            if test(c) {
                tx.send(c).unwrap();
            }
        });
    }
    drop(tx);
    rx.iter().min()
}

#[cfg(test)]
mod tests;
