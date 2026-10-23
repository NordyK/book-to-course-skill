use std::cmp::Ordering;

fn verdict(guess: u32, secret: u32) -> &'static str {
    match guess.cmp(&secret) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    }
}

fn parse_guess(line: &str) -> Option<u32> {
    match line.trim().parse::<u32>() {
        Ok(n) if (1..=100).contains(&n) => Some(n),
        _ => None,
    }
}

/// Plays a whole game with the given secret and typed lines,
/// returning every message the game prints, in order.
pub fn play(secret: u32, lines: &[&str]) -> Vec<String> {
    let mut output = Vec::new();
    for line in lines {
        match parse_guess(line) {
            None => output.push(String::from("Please type a number!")),
            Some(n) => {
                output.push(format!("You guessed: {n}"));
                output.push(verdict(n, secret).to_string());
                if n == secret {
                    break;
                }
            }
        }
    }
    output
}

#[cfg(test)]
mod tests;
