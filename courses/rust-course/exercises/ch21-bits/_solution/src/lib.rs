pub fn to_binary_i8(x: i8) -> String {
    format!("{:08b}", x as u8)
}

pub fn negate_manually(x: i8) -> i8 {
    (!x).wrapping_add(1)
}

pub fn add_policy(a: u8, b: u8, policy: &str) -> Option<u8> {
    match policy {
        "checked" => a.checked_add(b),
        "wrapping" => Some(a.wrapping_add(b)),
        "saturating" => Some(a.saturating_add(b)),
        _ => None,
    }
}

pub fn set_bit(x: u32, n: u32) -> u32 {
    x | (1 << n)
}

pub fn clear_bit(x: u32, n: u32) -> u32 {
    x & !(1 << n)
}

pub fn is_set(x: u32, n: u32) -> bool {
    x & (1 << n) != 0
}

pub fn swap_endian(x: u32) -> u32 {
    ((x & 0x0000_00FF) << 24)
        | ((x & 0x0000_FF00) << 8)
        | ((x & 0x00FF_0000) >> 8)
        | ((x & 0xFF00_0000) >> 24)
}

pub fn parse_be_u16s(bytes: &[u8]) -> Vec<u16> {
    bytes.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect()
}

#[cfg(test)]
mod tests;
