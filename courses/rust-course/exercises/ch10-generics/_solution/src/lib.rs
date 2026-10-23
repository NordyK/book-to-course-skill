/// The largest element, or None if the slice is empty.
pub fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut best = list.first()?;
    for item in list {
        if item > best {
            best = item;
        }
    }
    Some(best)
}

/// How many elements equal `target`.
pub fn count_matching<T: PartialEq>(list: &[T], target: &T) -> usize {
    list.iter().filter(|x| *x == target).count()
}

#[derive(Debug, PartialEq)]
pub struct Pair<T> {
    pub a: T,
    pub b: T,
}

impl<T> Pair<T> {
    pub fn new(a: T, b: T) -> Self {
        Pair { a, b }
    }

    pub fn swap(self) -> Pair<T> {
        Pair { a: self.b, b: self.a }
    }
}

impl<T: PartialOrd> Pair<T> {
    pub fn larger(&self) -> &T {
        if self.b > self.a { &self.b } else { &self.a }
    }
}

impl Pair<f64> {
    pub fn mean(&self) -> f64 {
        (self.a + self.b) / 2.0
    }
}

#[cfg(test)]
mod tests;
