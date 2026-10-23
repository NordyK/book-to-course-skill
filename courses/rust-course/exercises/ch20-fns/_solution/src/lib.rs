pub fn apply_n(f: fn(i64) -> i64, n: usize, x: i64) -> i64 {
    (0..n).fold(x, |acc, _| f(acc))
}

pub fn make_adder(k: i64) -> impl Fn(i64) -> i64 {
    move |x| x + k
}

pub fn make_counter() -> impl FnMut() -> u32 {
    let mut n = 0;
    move || {
        n += 1;
        n
    }
}

pub fn pipeline(steps: &[&str]) -> Box<dyn Fn(i64) -> i64> {
    let mut f: Box<dyn Fn(i64) -> i64> = Box::new(|x| x);
    for step in steps {
        let prev = f;
        f = match *step {
            "inc" => Box::new(move |x| prev(x) + 1),
            "double" => Box::new(move |x| prev(x) * 2),
            "neg" => Box::new(move |x| -prev(x)),
            _ => prev,
        };
    }
    f
}

pub fn longest_len<T: AsRef<str> + ?Sized>(items: &[&T]) -> usize {
    items.iter().map(|s| (*s).as_ref().len()).max().unwrap_or(0)
}

pub fn parse_or_exit(s: &str) -> i64 {
    match s.parse() {
        Ok(n) => n,
        Err(_) => panic!("bad number: {s}"),
    }
}

#[cfg(test)]
mod tests;
