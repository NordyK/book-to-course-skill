pub const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;

pub fn is_leap(y: i64) -> bool {
    todo!()
}

pub fn days_in_month(y: i64, m: u32) -> u32 {
    todo!()
}

pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    todo!()
}

pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    todo!()
}

pub fn to_rfc3339(unix: i64) -> String {
    todo!()
}

pub fn parse_rfc3339(s: &str) -> Option<i64> {
    todo!()
}

pub fn weekday(unix: i64) -> &'static str {
    todo!()
}

pub fn ntp_to_unix(seconds: u32, fraction: u32) -> (i64, u32) {
    todo!()
}

pub fn unix_to_ntp(secs: i64, nanos: u32) -> (u32, u32) {
    todo!()
}

pub fn ntp_offset_delay(t1: f64, t2: f64, t3: f64, t4: f64) -> (f64, f64) {
    todo!()
}

pub fn best_offset(samples: &[(f64, f64, f64, f64)]) -> Option<f64> {
    todo!()
}

#[cfg(test)]
mod tests;
