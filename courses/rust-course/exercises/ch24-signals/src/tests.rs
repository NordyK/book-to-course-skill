use super::*;

#[test]
fn numbers() {
    assert_eq!(from_number(2), Some(Signal::Int));
    assert_eq!(from_number(9), Some(Signal::Kill));
    assert_eq!(from_number(19), Some(Signal::Stop));
    assert_eq!(from_number(20), Some(Signal::Tstp));
    assert_eq!(from_number(5), None);
    assert_eq!(from_number(0), None);
    assert_eq!(from_number(-1), None);
    assert_eq!(Signal::Term as i32, 15);
}

#[test]
fn defaults() {
    use Action::*;
    for (s, a) in [
        (Signal::Term, Terminate), (Signal::Int, Terminate), (Signal::Kill, Terminate),
        (Signal::Usr1, Terminate), (Signal::Hup, Terminate), (Signal::Pipe, Terminate),
        (Signal::Segv, CoreDump), (Signal::Abrt, CoreDump), (Signal::Quit, CoreDump), (Signal::Fpe, CoreDump),
        (Signal::Chld, Ignore), (Signal::Stop, Stop), (Signal::Tstp, Stop), (Signal::Cont, Continue),
    ] {
        assert_eq!(default_action(s), a, "{s:?}");
    }
}

#[test]
fn default_delivery() {
    let mut p = Process::new();
    assert!(p.is_alive() && !p.is_stopped());
    assert_eq!(p.send(Signal::Chld), Outcome::Ignored);
    assert_eq!(p.send(Signal::Segv), Outcome::CoreDumped(Signal::Segv));
    assert!(!p.is_alive());
    assert_eq!(p.send(Signal::Term), Outcome::NoProcess);
}

#[test]
fn handlers_and_ignore() {
    let mut p = Process::new();
    p.set_disposition(Signal::Term, Disposition::Handle).unwrap();
    p.set_disposition(Signal::Int, Disposition::Ignore).unwrap();
    assert_eq!(p.send(Signal::Term), Outcome::Handled(Signal::Term));
    assert_eq!(p.send(Signal::Int), Outcome::Ignored);
    assert_eq!(p.send(Signal::Term), Outcome::Handled(Signal::Term));
    assert_eq!(p.handled(), &[Signal::Term, Signal::Term]);
    assert!(p.is_alive());
    p.set_disposition(Signal::Term, Disposition::Default).unwrap();
    assert_eq!(p.send(Signal::Term), Outcome::Terminated(Signal::Term));
    assert!(!p.is_alive());
    assert_eq!(p.send(Signal::Usr1), Outcome::NoProcess);
}

#[test]
fn kill_and_stop_are_special() {
    let mut p = Process::new();
    assert!(p.set_disposition(Signal::Kill, Disposition::Handle).is_err());
    assert!(p.set_disposition(Signal::Stop, Disposition::Ignore).is_err());
    assert!(p.set_disposition(Signal::Tstp, Disposition::Ignore).is_ok());
    p.block(Signal::Kill);
    p.block(Signal::Stop);
    assert_eq!(p.blocked_mask(), 0);
    assert_eq!(p.send(Signal::Stop), Outcome::Stopped);
    assert_eq!(p.send(Signal::Kill), Outcome::Terminated(Signal::Kill));
}

#[test]
fn stop_and_continue() {
    let mut p = Process::new();
    assert_eq!(p.send(Signal::Stop), Outcome::Stopped);
    assert!(p.is_stopped());
    assert_eq!(p.send(Signal::Cont), Outcome::Continued);
    assert!(!p.is_stopped());

    p.set_disposition(Signal::Cont, Disposition::Ignore).unwrap();
    assert_eq!(p.send(Signal::Tstp), Outcome::Stopped);
    assert_eq!(p.send(Signal::Cont), Outcome::Continued, "SIGCONT resumes even when ignored");
    assert!(!p.is_stopped());

    p.set_disposition(Signal::Cont, Disposition::Handle).unwrap();
    p.send(Signal::Stop);
    assert_eq!(p.send(Signal::Cont), Outcome::Handled(Signal::Cont));
    assert!(!p.is_stopped());
    assert_eq!(p.handled(), &[Signal::Cont]);
}

#[test]
fn blocking_and_pending() {
    let mut p = Process::new();
    p.set_disposition(Signal::Usr1, Disposition::Handle).unwrap();
    p.block(Signal::Usr1);
    assert_eq!(p.send(Signal::Usr1), Outcome::Pending);
    assert_eq!(p.send(Signal::Usr1), Outcome::Pending);
    assert!(p.is_pending(Signal::Usr1));
    assert!(p.handled().is_empty());
    assert_eq!(p.unblock(Signal::Usr1), Some(Outcome::Handled(Signal::Usr1)));
    assert_eq!(p.handled(), &[Signal::Usr1], "standard signals do not queue");
    assert!(!p.is_pending(Signal::Usr1));
    assert_eq!(p.unblock(Signal::Usr1), None);
    assert_eq!(p.send(Signal::Usr1), Outcome::Handled(Signal::Usr1));
}

#[test]
fn pending_default_acts_on_unblock() {
    let mut p = Process::new();
    p.block(Signal::Term);
    assert_eq!(p.send(Signal::Term), Outcome::Pending);
    assert!(p.is_alive());
    assert_eq!(p.unblock(Signal::Term), Some(Outcome::Terminated(Signal::Term)));
    assert!(!p.is_alive());
}

#[test]
fn mask() {
    let mut p = Process::new();
    p.block(Signal::Int);
    p.block(Signal::Term);
    p.block(Signal::Int);
    assert_eq!(p.blocked_mask(), 1 << 2 | 1 << 15);
    p.unblock(Signal::Int);
    assert_eq!(p.blocked_mask(), 1 << 15);
}
