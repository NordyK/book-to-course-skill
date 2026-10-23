/// One instruction for a tiny stack machine.
#[derive(Debug, PartialEq)]
pub enum Command {
    Push(i64),
    Pop,
    Add,
    Mul,
    Dup,
}

/// Turns one word into a command, if it is one.
pub fn parse(word: &str) -> Option<Command> {
    todo!()
}

/// Runs one command on the stack.
pub fn apply(stack: &mut Vec<i64>, cmd: Command) -> Result<(), String> {
    todo!()
}

/// Runs a whole program and returns the top of the stack.
pub fn eval(program: &str) -> Result<Option<i64>, String> {
    todo!()
}

#[cfg(test)]
mod tests;
