use std::collections::{BTreeMap, HashMap, HashSet};

/// Lowercased word -> how often it appears.
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// "Add <name> to <department>". Returns whether the command was valid.
pub fn add_employee(dir: &mut BTreeMap<String, Vec<String>>, command: &str) -> bool {
    todo!()
}

/// Everyone in the department, sorted.
pub fn department_list(dir: &BTreeMap<String, Vec<String>>, dept: &str) -> Vec<String> {
    todo!()
}

/// How many different characters the word has.
pub fn unique_letters(word: &str) -> usize {
    let _unused: HashSet<char> = HashSet::new();
    todo!()
}

#[cfg(test)]
mod tests;
