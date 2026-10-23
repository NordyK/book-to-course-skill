use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

pub fn grep<R: BufRead, W: Write>(input: R, pattern: &str, mut out: W) -> io::Result<usize> {
    let mut matches = 0;
    for (i, line) in input.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            matches += 1;
            writeln!(out, "{}:{}", i + 1, line)?;
        }
    }
    Ok(matches)
}

pub fn parse_config<R: BufRead>(input: R) -> io::Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("line {}: expected key = value", i + 1),
            ));
        };
        map.insert(k.trim().to_string(), v.trim().to_string());
    }
    Ok(map)
}

pub fn save_lines(path: &Path, lines: &[&str]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut w = BufWriter::new(fs::File::create(path)?);
    for line in lines {
        writeln!(w, "{line}")?;
    }
    w.flush()
}

pub fn load_lines(path: &Path) -> io::Result<Vec<String>> {
    match fs::File::open(path) {
        Ok(f) => io::BufReader::new(f).lines().collect(),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

pub fn largest_file(dir: &Path) -> io::Result<Option<(String, u64)>> {
    let mut best: Option<(String, u64)> = None;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if !meta.is_file() {
            continue;
        }
        let size = meta.len();
        if best.as_ref().map_or(true, |(_, b)| size > *b) {
            best = Some((entry.file_name().to_string_lossy().into_owned(), size));
        }
    }
    Ok(best)
}

#[cfg(test)]
mod tests;
