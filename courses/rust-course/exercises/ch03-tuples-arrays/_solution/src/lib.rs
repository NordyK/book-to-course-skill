/// The lowest and the highest temperature of the week: (min, max).
pub fn min_max(temps: [i32; 7]) -> (i32, i32) {
    let mut min = temps[0];
    let mut max = temps[0];
    for t in temps {
        if t < min {
            min = t;
        }
        if t > max {
            max = t;
        }
    }
    (min, max)
}

/// The index of the warmest day (0 = Monday). Ties go to the earliest day.
pub fn warmest_day(temps: [i32; 7]) -> usize {
    let mut best = 0;
    for i in 0..7 {
        if temps[i] > temps[best] {
            best = i;
        }
    }
    best
}

/// Returns the same values in the other order.
pub fn swap(pair: (String, i32)) -> (i32, String) {
    let (name, n) = pair;
    (n, name)
}

#[cfg(test)]
mod tests;
