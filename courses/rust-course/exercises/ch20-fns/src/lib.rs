pub fn apply_n(f: fn(i64) -> i64, n: usize, x: i64) -> i64 {
    todo!()
}

pub fn make_adder(k: i64) -> impl Fn(i64) -> i64 {
    move |x| todo!()
}

pub fn make_counter() -> impl FnMut() -> u32 {
    || todo!()
}

pub fn pipeline(steps: &[&str]) -> Box<dyn Fn(i64) -> i64> {
    todo!()
}

pub fn longest_len<T: AsRef<str> + ?Sized>(items: &[&T]) -> usize {
    todo!()
}

pub fn parse_or_exit(s: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests;
