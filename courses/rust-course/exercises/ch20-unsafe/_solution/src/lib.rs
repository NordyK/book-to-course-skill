pub fn split_at_mut<T>(v: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = v.len();
    assert!(mid <= len);
    let p = v.as_mut_ptr();
    // SAFETY: [0, mid) and [mid, len) are in bounds and disjoint
    unsafe {
        (
            std::slice::from_raw_parts_mut(p, mid),
            std::slice::from_raw_parts_mut(p.add(mid), len - mid),
        )
    }
}

pub fn first_last_mut<T>(v: &mut [T]) -> Option<(&mut T, &mut T)> {
    let len = v.len();
    if len < 2 {
        return None;
    }
    let p = v.as_mut_ptr();
    // SAFETY: len >= 2, so indices 0 and len - 1 are in bounds and distinct
    unsafe { Some((&mut *p, &mut *p.add(len - 1))) }
}

pub fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    if end > bytes.len() {
        return None;
    }
    // SAFETY: offset..offset + 4 is in bounds; read_unaligned has no alignment requirement
    let raw = unsafe { std::ptr::read_unaligned(bytes.as_ptr().add(offset) as *const u32) };
    Some(u32::from_le(raw))
}

/// Sums `len` integers starting at `ptr`.
///
/// # Safety
///
/// `ptr` must be valid for reads of `len` consecutive, initialized `i32` values
/// (it may dangle only when `len` is 0, and must still be non-null and aligned).
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    // SAFETY: guaranteed by the caller
    let s = unsafe { std::slice::from_raw_parts(ptr, len) };
    s.iter().sum()
}

pub fn sum(v: &[i32]) -> i32 {
    // SAFETY: a slice's pointer and length always satisfy sum_raw's contract
    unsafe { sum_raw(v.as_ptr(), v.len()) }
}

#[cfg(test)]
mod tests;
