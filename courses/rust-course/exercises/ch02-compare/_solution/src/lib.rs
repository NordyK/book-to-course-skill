use std::cmp::Ordering;

/// Returns what the game says for this guess.
pub fn verdict(guess: u32, secret: u32) -> &'static str {
    match guess.cmp(&secret) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    }
}

#[cfg(test)]
mod tests;
