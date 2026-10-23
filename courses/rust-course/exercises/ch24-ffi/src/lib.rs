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
    todo!()
}

pub fn c_div(n: i32, d: i32) -> Option<(i32, i32)> {
    todo!()
}

pub fn parse_long(s: &str, base: u32) -> Option<(i64, usize)> {
    todo!()
}

pub fn sort_desc(v: &mut [i32]) {
    todo!()
}

pub fn zero_pad(n: i32, width: usize) -> String {
    todo!()
}

#[cfg(test)]
mod tests;
