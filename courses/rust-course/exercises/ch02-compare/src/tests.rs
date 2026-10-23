use super::*;

#[test]
fn smaller_guess_is_too_small() {
    assert_eq!(verdict(10, 61), "Too small!", "guess 10, secret 61");
}

#[test]
fn bigger_guess_is_too_big() {
    assert_eq!(verdict(80, 61), "Too big!", "guess 80, secret 61");
}

#[test]
fn equal_guess_wins() {
    assert_eq!(verdict(61, 61), "You win!", "guess 61, secret 61");
}

#[test]
fn edges_of_the_range() {
    assert_eq!(verdict(1, 100), "Too small!", "guess 1, secret 100");
    assert_eq!(verdict(100, 1), "Too big!", "guess 100, secret 1");
    assert_eq!(verdict(1, 1), "You win!", "guess 1, secret 1");
}
