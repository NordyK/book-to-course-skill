/// The sum, or None if it does not fit in a u8.
pub fn add_checked(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
}

/// Turns the volume up by `step`, stopping at 255.
pub fn volume_up(level: u8, step: u8) -> u8 {
    level.saturating_add(step)
}

/// Adds `driven` kilometres, rolling over past 65,535 like a car odometer.
pub fn odometer(km: u16, driven: u16) -> u16 {
    km.wrapping_add(driven)
}

#[cfg(test)]
mod tests;
