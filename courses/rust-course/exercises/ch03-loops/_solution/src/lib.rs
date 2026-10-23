/// The n-th Fibonacci number: 0, 1, 1, 2, 3, 5, 8, ...
pub fn fibonacci(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

/// The sum of every number below `limit` that is a multiple of 3 or 5.
pub fn sum_of_multiples(limit: u32) -> u32 {
    let mut sum = 0;
    for i in 0..limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

/// How many decimal digits `n` has (0 has one digit).
pub fn digits(n: u64) -> u32 {
    let mut n = n;
    let mut count = 1;
    while n >= 10 {
        n /= 10;
        count += 1;
    }
    count
}

/// Is `n` a prime number?
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

#[cfg(test)]
mod tests;
