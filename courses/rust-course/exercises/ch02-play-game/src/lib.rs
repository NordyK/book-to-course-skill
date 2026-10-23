/// Plays a whole game with the given secret and typed lines,
/// returning every message the game prints, in order.
pub fn play(secret: u32, lines: &[&str]) -> Vec<String> {
    let mut output = Vec::new();
    for line in lines {
        // TODO: handle one typed line.
        // Add messages with output.push(String::from("...")) or output.push(format!(...)).
        let _ = (line, secret);
    }
    output
}

#[cfg(test)]
mod tests;
