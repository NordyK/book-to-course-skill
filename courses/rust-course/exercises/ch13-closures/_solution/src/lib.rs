/// A closure that computes g(f(x)).
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

/// Each call returns the next number, starting at 1.
pub fn counter() -> impl FnMut() -> u32 {
    let mut n = 0;
    move || {
        n += 1;
        n
    }
}

/// Calls op up to `times` times (at least once); first Ok wins, else the last Err.
pub fn retry<T, E>(times: usize, mut op: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    let mut last = op();
    for _ in 1..times {
        if last.is_ok() {
            break;
        }
        last = op();
    }
    last
}

/// Groups consecutive items with equal keys.
pub fn group_by_key<T, K: PartialEq>(items: Vec<T>, key: impl Fn(&T) -> K) -> Vec<(K, Vec<T>)> {
    let mut groups: Vec<(K, Vec<T>)> = Vec::new();
    for item in items {
        let k = key(&item);
        match groups.last_mut() {
            Some((last_k, members)) if *last_k == k => members.push(item),
            _ => groups.push((k, vec![item])),
        }
    }
    groups
}

#[cfg(test)]
mod tests;
