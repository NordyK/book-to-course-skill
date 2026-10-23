use std::thread;

pub fn parallel_sum(data: &[u64], parts: usize) -> u64 {
    todo!()
}

pub fn map_in_threads(inputs: Vec<String>, f: fn(&str) -> String) -> Vec<String> {
    todo!()
}

pub fn run_safely(job: fn() -> i32) -> Result<i32, String> {
    todo!()
}

#[cfg(test)]
mod tests;
