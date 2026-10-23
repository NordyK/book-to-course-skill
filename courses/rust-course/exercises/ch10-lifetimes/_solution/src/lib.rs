pub fn longest_word<'a>(text: &'a str) -> &'a str {
    let mut best = "";
    for w in text.split_whitespace() {
        if w.len() > best.len() {
            best = w;
        }
    }
    best
}

pub fn longer_of<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() { b } else { a }
}

pub fn after_prefix<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    text.strip_prefix(prefix)
}

pub struct Tokens<'a> {
    rest: &'a str,
}

impl<'a> Tokens<'a> {
    pub fn new(text: &'a str) -> Self {
        Tokens { rest: text }
    }

    pub fn next_token(&mut self) -> Option<&'a str> {
        loop {
            if self.rest.is_empty() {
                return None;
            }
            let piece = match self.rest.split_once(',') {
                Some((head, tail)) => {
                    self.rest = tail;
                    head
                }
                None => {
                    let all = self.rest;
                    self.rest = "";
                    all
                }
            };
            let piece = piece.trim();
            if !piece.is_empty() {
                return Some(piece);
            }
        }
    }
}

#[cfg(test)]
mod tests;
