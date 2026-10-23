use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

pub fn grep<R: BufRead, W: Write>(input: R, pattern: &str, mut out: W) -> io::Result<usize> {
    todo!()
}

pub fn parse_config<R: BufRead>(input: R) -> io::Result<HashMap<String, String>> {
    todo!()
}

pub fn save_lines(path: &Path, lines: &[&str]) -> io::Result<()> {
    todo!()
}

pub fn load_lines(path: &Path) -> io::Result<Vec<String>> {
    todo!()
}

pub fn largest_file(dir: &Path) -> io::Result<Option<(String, u64)>> {
    todo!()
}

#[cfg(test)]
mod tests;
