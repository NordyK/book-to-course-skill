/// Returns what the game says for this guess.
pub fn verdict(guess: u32, secret: u32) -> &'static str {
    // TODO: compare `guess` with `secret` and return
    // "Too small!", "Too big!" or "You win!".
    todo!()
}

#[cfg(test)]
mod tests;
