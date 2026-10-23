/// Everything before the first space.
pub fn first_word(s: &str) -> &str {
    &s[..s.find(' ').unwrap_or(s.len())]
}

/// The n-th word (0-based), if there is one.
pub fn nth_word(s: &str, n: usize) -> Option<&str> {
    let mut i = 0;
    for word in s.split(' ') {
        if i == n {
            return Some(word);
        }
        i += 1;
    }
    None
}

/// The largest value, or None when the slice is empty.
pub fn largest(values: &[i32]) -> Option<i32> {
    if values.is_empty() {
        return None;
    }
    let mut max = values[0];
    for &v in values {
        if v > max {
            max = v;
        }
    }
    Some(max)
}

/// Sets every negative value to 0.
pub fn zero_negatives(values: &mut [i32]) {
    for v in values {
        if *v < 0 {
            *v = 0;
        }
    }
}

#[cfg(test)]
mod tests;
