use super::*;

#[test]
fn new_account_is_empty() {
    let a = Account::new("Ada");
    assert_eq!(a.owner(), "Ada");
    assert_eq!(a.balance(), 0);
    assert!(a.history().is_empty(), "no history yet");
}

#[test]
fn deposits_and_withdrawals() {
    let mut a = Account::new("Ada");
    a.deposit(100);
    assert!(a.withdraw(30), "withdrawing 30 of 100 works");
    assert_eq!(a.balance(), 70);
    assert_eq!(a.history(), &[100, -30]);
}

#[test]
fn cannot_overdraw() {
    let mut a = Account::new("Ada");
    a.deposit(50);
    assert!(!a.withdraw(51), "withdrawing more than the balance fails");
    assert_eq!(a.balance(), 50, "a failed withdrawal changes nothing");
    assert_eq!(a.history(), &[50], "a failed withdrawal is not recorded");
    assert!(a.withdraw(50), "withdrawing exactly the balance works");
    assert_eq!(a.balance(), 0);
}

#[test]
fn closing_returns_the_balance() {
    let mut a = Account::new("Ada");
    a.deposit(7);
    assert_eq!(a.close(), 7);
}
