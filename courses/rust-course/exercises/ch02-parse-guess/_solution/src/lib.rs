/// Turns one typed line into a guess from 1 to 100, or None if it is not one.
pub fn parse_guess(line: &str) -> Option<u32> {
    match line.trim().parse::<u32>() {
        Ok(n) => {
            if n >= 1 && n <= 100 {
                Some(n)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests;
