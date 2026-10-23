use std::num::ParseIntError;

pub fn word_lengths(text: &str) -> Vec<(String, usize)> {
    todo!()
}

pub fn top_n(scores: &[(String, u32)], n: usize) -> Vec<String> {
    todo!()
}

pub fn running_total(values: &[i64]) -> Vec<i64> {
    todo!()
}

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    todo!()
}

pub struct Collatz {
    n: u64,
    done: bool,
}

pub fn collatz(start: u64) -> Collatz {
    Collatz { n: start, done: false }
}

// impl Iterator for Collatz { ... }

#[cfg(test)]
mod tests;
