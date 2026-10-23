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
    use Signal::*;
    Some(match n {
        1 => Hup, 2 => Int, 3 => Quit, 4 => Ill, 6 => Abrt, 8 => Fpe, 9 => Kill, 10 => Usr1,
        11 => Segv, 12 => Usr2, 13 => Pipe, 14 => Alrm, 15 => Term, 17 => Chld, 18 => Cont,
        19 => Stop, 20 => Tstp,
        _ => return None,
    })
}

pub fn default_action(s: Signal) -> Action {
    use Signal::*;
    match s {
        Hup | Int | Kill | Usr1 | Usr2 | Pipe | Alrm | Term => Action::Terminate,
        Quit | Ill | Abrt | Fpe | Segv => Action::CoreDump,
        Chld => Action::Ignore,
        Stop | Tstp => Action::Stop,
        Cont => Action::Continue,
    }
}

fn bit(s: Signal) -> u32 {
    1u32 << (s as u32)
}

fn uncatchable(s: Signal) -> bool {
    matches!(s, Signal::Kill | Signal::Stop)
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
        Process {
            dispositions: [Disposition::Default; 32],
            blocked: 0,
            pending: 0,
            alive: true,
            stopped: false,
            handled: Vec::new(),
        }
    }

    pub fn set_disposition(&mut self, s: Signal, d: Disposition) -> Result<(), &'static str> {
        if uncatchable(s) {
            return Err("SIGKILL and SIGSTOP cannot be caught or ignored");
        }
        self.dispositions[s as usize] = d;
        Ok(())
    }

    pub fn block(&mut self, s: Signal) {
        if !uncatchable(s) {
            self.blocked |= bit(s);
        }
    }

    pub fn unblock(&mut self, s: Signal) -> Option<Outcome> {
        self.blocked &= !bit(s);
        if self.pending & bit(s) != 0 {
            self.pending &= !bit(s);
            Some(self.deliver(s))
        } else {
            None
        }
    }

    pub fn blocked_mask(&self) -> u32 {
        self.blocked
    }

    pub fn send(&mut self, s: Signal) -> Outcome {
        if !self.alive {
            return Outcome::NoProcess;
        }
        if self.blocked & bit(s) != 0 {
            self.pending |= bit(s);
            return Outcome::Pending;
        }
        self.deliver(s)
    }

    fn deliver(&mut self, s: Signal) -> Outcome {
        let disposition = self.dispositions[s as usize];
        if s == Signal::Cont {
            self.stopped = false;
            if disposition == Disposition::Handle {
                self.handled.push(s);
                return Outcome::Handled(s);
            }
            return Outcome::Continued;
        }
        match disposition {
            Disposition::Ignore => Outcome::Ignored,
            Disposition::Handle => {
                self.handled.push(s);
                Outcome::Handled(s)
            }
            Disposition::Default => match default_action(s) {
                Action::Terminate => {
                    self.alive = false;
                    Outcome::Terminated(s)
                }
                Action::CoreDump => {
                    self.alive = false;
                    Outcome::CoreDumped(s)
                }
                Action::Ignore => Outcome::Ignored,
                Action::Stop => {
                    self.stopped = true;
                    Outcome::Stopped
                }
                Action::Continue => Outcome::Continued,
            },
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn is_stopped(&self) -> bool {
        self.stopped
    }

    pub fn is_pending(&self, s: Signal) -> bool {
        self.pending & bit(s) != 0
    }

    pub fn handled(&self) -> &[Signal] {
        &self.handled
    }
}

#[cfg(test)]
mod tests;
