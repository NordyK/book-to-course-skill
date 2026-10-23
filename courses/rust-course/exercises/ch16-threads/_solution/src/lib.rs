use std::thread;

pub fn parallel_sum(data: &[u64], parts: usize) -> u64 {
    let size = data.len().div_ceil(parts.max(1)).max(1);
    thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(size)
            .map(|c| s.spawn(move || c.iter().sum::<u64>()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}

pub fn map_in_threads(inputs: Vec<String>, f: fn(&str) -> String) -> Vec<String> {
    let handles: Vec<_> = inputs
        .into_iter()
        .map(|s| thread::spawn(move || f(&s)))
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

pub fn run_safely(job: fn() -> i32) -> Result<i32, String> {
    thread::spawn(job).join().map_err(|_| String::from("job panicked"))
}

#[cfg(test)]
mod tests;
