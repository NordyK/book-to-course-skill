use std::collections::{BTreeMap, HashMap, HashSet};

/// Lowercased word -> how often it appears.
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// "Add <name> to <department>". Returns whether the command was valid.
pub fn add_employee(dir: &mut BTreeMap<String, Vec<String>>, command: &str) -> bool {
    let words: Vec<&str> = command.split_whitespace().collect();
    if let ["Add", name, "to", dept] = words.as_slice() {
        dir.entry(dept.to_string()).or_default().push(name.to_string());
        true
    } else {
        false
    }
}

/// Everyone in the department, sorted.
pub fn department_list(dir: &BTreeMap<String, Vec<String>>, dept: &str) -> Vec<String> {
    let mut names = dir.get(dept).cloned().unwrap_or_default();
    names.sort();
    names
}

/// How many different characters the word has.
pub fn unique_letters(word: &str) -> usize {
    let set: HashSet<char> = word.chars().collect();
    set.len()
}

#[cfg(test)]
mod tests;
