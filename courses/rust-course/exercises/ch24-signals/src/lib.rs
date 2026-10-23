#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Hup = 1, Int = 2, Quit = 3, Ill = 4, Abrt = 6, Fpe = 8, Kill = 9, Usr1 = 10,
    Segv = 11, Usr2 = 12, Pipe = 13, Alrm = 14, Term = 15, Chld = 17, Cont = 18, Stop = 19, Tstp = 20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Terminate,
    CoreDump,
    Ignore,
    Stop,
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    Default,
    Ignore,
    Handle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Terminated(Signal),
    CoreDumped(Signal),
    Stopped,
    Continued,
    Ignored,
    Handled(Signal),
    Pending,
    NoProcess,
}

pub fn from_number(n: i32) -> Option<Signal> {
    todo!()
}

pub fn default_action(s: Signal) -> Action {
    todo!()
}

pub struct Process {
    dispositions: [Disposition; 32],
    blocked: u32,
    pending: u32,
    alive: bool,
    stopped: bool,
    handled: Vec<Signal>,
}

impl Process {
    pub fn new() -> Process {
        todo!()
    }

    pub fn set_disposition(&mut self, s: Signal, d: Disposition) -> Result<(), &'static str> {
        todo!()
    }

    pub fn block(&mut self, s: Signal) {
        todo!()
    }

    pub fn unblock(&mut self, s: Signal) -> Option<Outcome> {
        todo!()
    }

    pub fn blocked_mask(&self) -> u32 {
        todo!()
    }

    pub fn send(&mut self, s: Signal) -> Outcome {
        todo!()
    }

    pub fn is_alive(&self) -> bool {
        todo!()
    }

    pub fn is_stopped(&self) -> bool {
        todo!()
    }

    pub fn is_pending(&self, s: Signal) -> bool {
        todo!()
    }

    pub fn handled(&self) -> &[Signal] {
        todo!()
    }
}

#[cfg(test)]
mod tests;
