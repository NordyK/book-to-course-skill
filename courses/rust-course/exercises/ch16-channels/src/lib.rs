use std::sync::mpsc;
use std::thread;

pub fn fan_out(jobs: Vec<u64>, workers: usize, f: fn(u64) -> u64) -> Vec<u64> {
    todo!()
}

pub fn first_match(candidates: Vec<u64>, test: fn(u64) -> bool) -> Option<u64> {
    todo!()
}

#[cfg(test)]
mod tests;
