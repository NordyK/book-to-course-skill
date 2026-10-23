/// A closure that computes g(f(x)).
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| todo!()
}

/// Each call returns the next number, starting at 1.
pub fn counter() -> impl FnMut() -> u32 {
    || todo!()
}

/// Calls op up to `times` times (at least once); first Ok wins, else the last Err.
pub fn retry<T, E>(times: usize, mut op: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    todo!()
}

/// Groups consecutive items with equal keys.
pub fn group_by_key<T, K: PartialEq>(items: Vec<T>, key: impl Fn(&T) -> K) -> Vec<(K, Vec<T>)> {
    todo!()
}

#[cfg(test)]
mod tests;
