/// The even numbers, in order.
pub fn evens(values: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    for &v in values {
        if v % 2 == 0 {
            out.push(v);
        }
    }
    out
}

/// Element i is values[0] + ... + values[i].
pub fn running_total(values: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(values.len());
    let mut sum = 0;
    for &v in values {
        sum += v;
        out.push(sum);
    }
    out
}

/// Keeps the first occurrence of each value.
pub fn remove_duplicates(values: &mut Vec<i32>) {
    let mut seen = Vec::new();
    for &v in values.iter() {
        if !seen.contains(&v) {
            seen.push(v);
        }
    }
    *values = seen;
}

/// The median, or None for no values.
pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

#[cfg(test)]
mod tests;
