/// Wraps `text` into lines of at most `width` characters.
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in words(text) {
        if fits(&current, word, width) {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        } else {
            if !current.is_empty() {
                lines.push(current);
            }
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn words(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

fn fits(line: &str, word: &str, width: usize) -> bool {
    let w = word.chars().count();
    if line.is_empty() {
        w <= width
    } else {
        line.chars().count() + 1 + w <= width
    }
}

#[cfg(test)]
mod tests;
