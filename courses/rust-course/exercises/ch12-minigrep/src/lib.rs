use std::io::{self, Write};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String], ignore_case_env: bool) -> Result<Config, &'static str> {
        todo!()
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    todo!()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    todo!()
}

pub fn run_on<W: Write>(config: &Config, contents: &str, out: &mut W) -> io::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests;
