/// The text in capital letters with "!" at the end.
pub fn shout(s: String) -> String {
    let mut loud = s.to_uppercase();
    loud.push('!');
    loud
}

/// "a b": the two words joined by one space.
pub fn join_words(mut a: String, b: String) -> String {
    a.push(' ');
    a.push_str(&b);
    a
}

/// Two separate Strings holding the same text.
pub fn twice(s: &str) -> (String, String) {
    let first = String::from(s);
    let second = first.clone();
    (first, second)
}

#[cfg(test)]
mod tests;
