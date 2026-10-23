/// "key = value" -> Some((key, value)), trimmed. Comments, blank lines and lines without '=' are None.
pub fn parse_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let Some((k, v)) = line.split_once('=') else {
        return None;
    };
    let k = k.trim();
    if k.is_empty() {
        return None;
    }
    Some((k.to_string(), v.trim().to_string()))
}

/// The value of `port`, or 8080 when it is missing or invalid.
pub fn port(lines: &[&str]) -> u16 {
    for line in lines {
        if let Some((k, v)) = parse_line(line) {
            if k == "port" {
                if let Ok(p) = v.parse::<u16>() {
                    return p;
                }
            }
        }
    }
    8080
}

/// How many lines are valid settings.
pub fn count_settings(lines: &[&str]) -> usize {
    let mut n = 0;
    for line in lines {
        if parse_line(line).is_some() {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
mod tests;
