/// n times n.
pub fn square(n: i64) -> i64 {
    n * n
}

/// The hypotenuse of a right triangle with sides a and b.
pub fn hypotenuse(a: f64, b: f64) -> f64 {
    (a * a + b * b).sqrt()
}

/// Limits a score to 0..=100.
pub fn clamp_score(score: i32) -> i32 {
    if score < 0 {
        0
    } else if score > 100 {
        100
    } else {
        score
    }
}

/// 'A' (90+), 'B' (80+), 'C' (70+) or 'F', after clamping the score.
pub fn grade(score: i32) -> char {
    let s = clamp_score(score);
    if s >= 90 {
        'A'
    } else if s >= 80 {
        'B'
    } else if s >= 70 {
        'C'
    } else {
        'F'
    }
}

#[cfg(test)]
mod tests;
