/// The largest element, or None if the slice is empty.
pub fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    todo!()
}

/// How many elements equal `target`.
pub fn count_matching<T: PartialEq>(list: &[T], target: &T) -> usize {
    todo!()
}

#[derive(Debug, PartialEq)]
pub struct Pair<T> {
    pub a: T,
    pub b: T,
}

impl<T> Pair<T> {
    pub fn new(a: T, b: T) -> Self {
        todo!()
    }

    pub fn swap(self) -> Pair<T> {
        todo!()
    }
}

// impl<T: PartialOrd> Pair<T> { pub fn larger(&self) -> &T }

// impl Pair<f64> { pub fn mean(&self) -> f64 }

#[cfg(test)]
mod tests;
