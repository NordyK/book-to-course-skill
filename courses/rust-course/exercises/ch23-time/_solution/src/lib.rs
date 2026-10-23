pub const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;

pub fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

pub fn days_in_month(y: i64, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(y) => 29,
        2 => 28,
        _ => 0,
    }
}

pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub fn to_rfc3339(unix: i64) -> String {
    let (days, secs) = (unix.div_euclid(86_400), unix.rem_euclid(86_400));
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", secs / 3600, secs / 60 % 60, secs % 60)
}

pub fn parse_rfc3339(s: &str) -> Option<i64> {
    let b = s.as_bytes();
    if b.len() != 20 {
        return None;
    }
    for (i, c) in [(4, b'-'), (7, b'-'), (10, b'T'), (13, b':'), (16, b':'), (19, b'Z')] {
        if b[i] != c {
            return None;
        }
    }
    let num = |r: std::ops::Range<usize>| -> Option<i64> {
        let part = s.get(r)?;
        if !part.bytes().all(|c| c.is_ascii_digit()) {
            return None;
        }
        part.parse().ok()
    };
    let (y, mo, d) = (num(0..4)?, num(5..7)? as u32, num(8..10)? as u32);
    let (h, mi, se) = (num(11..13)?, num(14..16)?, num(17..19)?);
    if !(1..=12).contains(&mo) || d == 0 || d > days_in_month(y, mo) || h >= 24 || mi >= 60 || se >= 60 {
        return None;
    }
    Some(days_from_civil(y, mo, d) * 86_400 + h * 3600 + mi * 60 + se)
}

pub fn weekday(unix: i64) -> &'static str {
    const NAMES: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    NAMES[(unix.div_euclid(86_400) + 3).rem_euclid(7) as usize]
}

pub fn ntp_to_unix(seconds: u32, fraction: u32) -> (i64, u32) {
    let nanos = ((fraction as u64) * 1_000_000_000) >> 32;
    (seconds as i64 - NTP_TO_UNIX_SECONDS, nanos as u32)
}

pub fn unix_to_ntp(secs: i64, nanos: u32) -> (u32, u32) {
    let fraction = ((nanos as u64) << 32) / 1_000_000_000;
    ((secs + NTP_TO_UNIX_SECONDS) as u32, fraction as u32)
}

pub fn ntp_offset_delay(t1: f64, t2: f64, t3: f64, t4: f64) -> (f64, f64) {
    (((t2 - t1) + (t3 - t4)) / 2.0, (t4 - t1) - (t3 - t2))
}

pub fn best_offset(samples: &[(f64, f64, f64, f64)]) -> Option<f64> {
    let mut total = 0.0;
    let mut weights = 0.0;
    for &(t1, t2, t3, t4) in samples {
        let (offset, delay) = ntp_offset_delay(t1, t2, t3, t4);
        if delay <= 0.0 {
            continue;
        }
        let w = 1.0 / (delay * delay);
        total += offset * w;
        weights += w;
    }
    if weights > 0.0 { Some(total / weights) } else { None }
}

#[cfg(test)]
mod tests;
