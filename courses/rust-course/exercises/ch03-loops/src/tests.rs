use super::*;

#[test]
fn fibonacci_start() {
    assert_eq!(fibonacci(0), 0, "fibonacci(0)");
    assert_eq!(fibonacci(1), 1, "fibonacci(1)");
    assert_eq!(fibonacci(2), 1, "fibonacci(2)");
    assert_eq!(fibonacci(10), 55, "fibonacci(10)");
}

#[test]
fn fibonacci_large() {
    assert_eq!(fibonacci(90), 2_880_067_194_370_816_120, "fibonacci(90) must not overflow u64");
}

#[test]
fn multiples() {
    assert_eq!(sum_of_multiples(10), 23, "sum_of_multiples(10)");
    assert_eq!(sum_of_multiples(16), 60, "15 is counted once");
    assert_eq!(sum_of_multiples(1), 0, "sum_of_multiples(1)");
    assert_eq!(sum_of_multiples(1000), 233_168, "sum_of_multiples(1000)");
}

#[test]
fn digit_counts() {
    assert_eq!(digits(0), 1, "digits(0)");
    assert_eq!(digits(9), 1, "digits(9)");
    assert_eq!(digits(10), 2, "digits(10)");
    assert_eq!(digits(12345), 5, "digits(12345)");
    assert_eq!(digits(u64::MAX), 20, "digits(u64::MAX)");
}

#[test]
fn primes() {
    for p in [2, 3, 5, 7, 11, 97, 7919] {
        assert!(is_prime(p), "{p} is prime");
    }
    for n in [0, 1, 4, 9, 15, 91, 7917] {
        assert!(!is_prime(n), "{n} is not prime");
    }
}
