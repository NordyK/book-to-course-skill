use super::*;

#[test]
fn split() {
    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];
    let (a, b) = split_at_mut(&mut v, 1);
    a[0].push('!');
    b[1].push('?');
    assert_eq!(v, ["a!", "b", "c?"]);
    let mut e: Vec<i32> = vec![];
    let (x, y) = split_at_mut(&mut e, 0);
    assert!(x.is_empty() && y.is_empty());
}

#[test]
#[should_panic]
fn split_out_of_range() {
    let mut v = [1, 2, 3];
    split_at_mut(&mut v, 4);
}

#[test]
fn ends() {
    let mut v = [1, 2, 3, 4];
    if let Some((a, b)) = first_last_mut(&mut v) {
        std::mem::swap(a, b);
    }
    assert_eq!(v, [4, 2, 3, 1]);
    assert!(first_last_mut(&mut [9]).is_none());
    assert!(first_last_mut::<i32>(&mut []).is_none());
}

#[test]
fn reading() {
    let bytes = [0x78, 0x56, 0x34, 0x12, 0xff, 0x01, 0, 0, 0];
    assert_eq!(read_u32_le(&bytes, 0), Some(0x12345678));
    assert_eq!(read_u32_le(&bytes, 5), Some(1), "unaligned offset");
    assert_eq!(read_u32_le(&bytes, 6), None);
    assert_eq!(read_u32_le(&bytes, usize::MAX), None, "offset + 4 must not overflow");
}

#[test]
fn sums() {
    assert_eq!(sum(&[1, 2, 3, 4]), 10);
    assert_eq!(sum(&[]), 0);
    let v = [5, 5];
    assert_eq!(unsafe { sum_raw(v.as_ptr(), 2) }, 10);
}
