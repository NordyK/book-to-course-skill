/// Number of chars (Unicode scalar values).
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// apple -> apple-hay, first -> irst-fay.
pub fn pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(c) if "aeiou".contains(c) => format!("{word}-hay"),
        Some(c) => format!("{}-{c}ay", chars.as_str()),
    }
}

/// At most `max` chars, ending with '…' if anything was cut.
pub fn truncate_chars(s: &str, max: usize) -> String {
    if char_count(s) <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max - 1).collect();
    out.push('…');
    out
}

/// Number of whitespace-separated words.
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

#[cfg(test)]
mod tests;
