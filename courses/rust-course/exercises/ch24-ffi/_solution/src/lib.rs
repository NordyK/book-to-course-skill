use std::cmp::Ordering;
use std::ffi::{CStr, CString, c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct DivT {
    pub quot: c_int,
    pub rem: c_int,
}

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn div(n: c_int, d: c_int) -> DivT;
    fn strtol(s: *const c_char, end: *mut *mut c_char, base: c_int) -> c_long;
    fn qsort(base: *mut c_void, n: usize, size: usize, cmp: extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn snprintf(buf: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
}

pub fn c_len(s: &CStr) -> usize {
    // SAFETY: a CStr is always NUL-terminated.
    unsafe { strlen(s.as_ptr()) }
}

pub fn c_div(n: i32, d: i32) -> Option<(i32, i32)> {
    if d == 0 || (n == i32::MIN && d == -1) {
        return None;
    }
    // SAFETY: the two undefined cases were excluded above.
    let r = unsafe { div(n, d) };
    Some((r.quot, r.rem))
}

pub fn parse_long(s: &str, base: u32) -> Option<(i64, usize)> {
    if base == 1 || base > 36 {
        return None;
    }
    let c = CString::new(s).ok()?;
    let mut end: *mut c_char = std::ptr::null_mut();
    // SAFETY: c is NUL-terminated and outlives the call; end points into c afterwards.
    let value = unsafe { strtol(c.as_ptr(), &mut end, base as c_int) };
    let used = end as usize - c.as_ptr() as usize;
    if used == 0 { None } else { Some((value as i64, used)) }
}

extern "C" fn descending(a: *const c_void, b: *const c_void) -> c_int {
    // SAFETY: qsort passes pointers to elements of an i32 slice.
    let (a, b) = unsafe { (*(a as *const i32), *(b as *const i32)) };
    match b.cmp(&a) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub fn sort_desc(v: &mut [i32]) {
    if v.is_empty() {
        return;
    }
    // SAFETY: the pointer, count and element size describe the slice exactly.
    unsafe { qsort(v.as_mut_ptr().cast(), v.len(), size_of::<i32>(), descending) };
}

pub fn zero_pad(n: i32, width: usize) -> String {
    let fmt = c"%0*d";
    let width = width as c_int;
    // SAFETY: a NULL buffer with size 0 is allowed; the arguments match %0*d (int, int).
    let len = unsafe { snprintf(std::ptr::null_mut(), 0, fmt.as_ptr(), width, n as c_int) };
    let len = len as usize;
    let mut buf = vec![0u8; len + 1];
    // SAFETY: buf has room for len characters plus the NUL.
    unsafe { snprintf(buf.as_mut_ptr().cast(), buf.len(), fmt.as_ptr(), width, n as c_int) };
    buf.truncate(len);
    String::from_utf8(buf).expect("digits are ASCII")
}

#[cfg(test)]
mod tests;
