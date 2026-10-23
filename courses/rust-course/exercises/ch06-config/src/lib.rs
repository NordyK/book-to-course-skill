/// "key = value" -> Some((key, value)), trimmed. Comments, blank lines and lines without '=' are None.
pub fn parse_line(line: &str) -> Option<(String, String)> {
    todo!()
}

/// The value of `port`, or 8080 when it is missing or invalid.
pub fn port(lines: &[&str]) -> u16 {
    todo!()
}

/// How many lines are valid settings.
pub fn count_settings(lines: &[&str]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests;
