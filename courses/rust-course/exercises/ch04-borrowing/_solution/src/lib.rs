/// Counts a, e, i, o, u in either case.
pub fn count_vowels(s: &String) -> usize {
    let mut count = 0;
    for c in s.chars() {
        if "aeiouAEIOU".contains(c) {
            count += 1;
        }
    }
    count
}

/// Uppercases the first character, in place.
pub fn make_title(s: &mut String) {
    if s.is_empty() {
        return;
    }
    let first = s.remove(0);
    s.insert_str(0, &first.to_uppercase().to_string());
}

/// Adds the three points to the total.
pub fn add_points(total: &mut u32, points: &[u32; 3]) {
    for p in points {
        *total += p;
    }
}

/// The longer String; `a` when they are the same length.
pub fn longer<'a>(a: &'a String, b: &'a String) -> &'a String {
    if b.len() > a.len() { b } else { a }
}

#[cfg(test)]
mod tests;
