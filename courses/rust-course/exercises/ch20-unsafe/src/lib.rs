pub fn split_at_mut<T>(v: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    todo!()
}

pub fn first_last_mut<T>(v: &mut [T]) -> Option<(&mut T, &mut T)> {
    todo!()
}

pub fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32> {
    todo!()
}

/// Sums `len` integers starting at `ptr`.
///
/// # Safety
///
/// TODO: write the contract
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    todo!()
}

pub fn sum(v: &[i32]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests;
