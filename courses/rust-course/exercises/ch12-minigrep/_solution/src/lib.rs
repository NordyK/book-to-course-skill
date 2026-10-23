use std::io::{self, Write};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String], ignore_case_env: bool) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = match args.get(3).map(|s| s.as_str()) {
            None => ignore_case_env,
            Some("-i") => true,
            Some("-s") => false,
            Some(_) => return Err("unknown option"),
        };
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn run_on<W: Write>(config: &Config, contents: &str, out: &mut W) -> io::Result<usize> {
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, contents)
    } else {
        search(&config.query, contents)
    };
    for line in &results {
        writeln!(out, "{line}")?;
    }
    Ok(results.len())
}

#[cfg(test)]
mod tests;
