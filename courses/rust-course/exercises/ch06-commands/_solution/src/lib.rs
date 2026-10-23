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
    match word {
        "pop" => Some(Command::Pop),
        "add" => Some(Command::Add),
        "mul" => Some(Command::Mul),
        "dup" => Some(Command::Dup),
        _ => word.parse().ok().map(Command::Push),
    }
}

/// Runs one command on the stack.
pub fn apply(stack: &mut Vec<i64>, cmd: Command) -> Result<(), String> {
    let needed = match cmd {
        Command::Push(_) => 0,
        Command::Pop | Command::Dup => 1,
        Command::Add | Command::Mul => 2,
    };
    if stack.len() < needed {
        return Err(String::from("stack underflow"));
    }
    match cmd {
        Command::Push(n) => stack.push(n),
        Command::Pop => {
            stack.pop();
        }
        Command::Dup => {
            let top = stack[stack.len() - 1];
            stack.push(top);
        }
        Command::Add | Command::Mul => {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(if cmd == Command::Add { a + b } else { a * b });
        }
    }
    Ok(())
}

/// Runs a whole program and returns the top of the stack.
pub fn eval(program: &str) -> Result<Option<i64>, String> {
    let mut stack = Vec::new();
    for w in program.split_whitespace() {
        match parse(w) {
            Some(cmd) => apply(&mut stack, cmd)?,
            None => return Err(format!("unknown word: {w}")),
        }
    }
    Ok(stack.last().copied())
}

#[cfg(test)]
mod tests;
