use std::num::ParseIntError;

pub fn word_lengths(text: &str) -> Vec<(String, usize)> {
    text.split_whitespace()
        .map(|w| {
            let w = w.to_lowercase();
            let n = w.chars().count();
            (w, n)
        })
        .collect()
}

pub fn top_n(scores: &[(String, u32)], n: usize) -> Vec<String> {
    let mut sorted: Vec<&(String, u32)> = scores.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.iter().take(n).map(|s| s.0.clone()).collect()
}

pub fn running_total(values: &[i64]) -> Vec<i64> {
    values
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    items.iter().map(|s| s.parse::<i32>()).collect()
}

pub struct Collatz {
    n: u64,
    done: bool,
}

pub fn collatz(start: u64) -> Collatz {
    Collatz { n: start, done: false }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.done {
            return None;
        }
        let current = self.n;
        if current == 1 {
            self.done = true;
        } else if current % 2 == 0 {
            self.n = current / 2;
        } else {
            self.n = 3 * current + 1;
        }
        Some(current)
    }
}

#[cfg(test)]
mod tests;
