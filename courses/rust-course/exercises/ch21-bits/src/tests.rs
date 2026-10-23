use super::*;

#[test]
fn binary() {
    assert_eq!(to_binary_i8(5), "00000101");
    assert_eq!(to_binary_i8(-5), "11111011");
    assert_eq!(to_binary_i8(-128), "10000000");
    assert_eq!(to_binary_i8(-1), "11111111");
}

#[test]
fn negation() {
    for x in [-127i8, -5, 0, 1, 42, 127] {
        assert_eq!(negate_manually(x), -x);
    }
    assert_eq!(negate_manually(i8::MIN), i8::MIN);
}

#[test]
fn policies() {
    assert_eq!(add_policy(250, 10, "checked"), None);
    assert_eq!(add_policy(250, 5, "checked"), Some(255));
    assert_eq!(add_policy(250, 10, "wrapping"), Some(4));
    assert_eq!(add_policy(250, 10, "saturating"), Some(255));
    assert_eq!(add_policy(1, 1, "magic"), None);
}

#[test]
fn single_bits() {
    assert_eq!(set_bit(0, 3), 8);
    assert_eq!(set_bit(8, 3), 8);
    assert_eq!(clear_bit(0b1111, 0), 0b1110);
    assert!(is_set(0b100, 2));
    assert!(!is_set(0b100, 1));
    assert_eq!(set_bit(0, 31), 0x8000_0000);
}

#[test]
fn endianness() {
    assert_eq!(swap_endian(0x1234_5678), 0x7856_3412);
    assert_eq!(swap_endian(0xFF00_0000), 0x0000_00FF);
    assert_eq!(swap_endian(swap_endian(0xDEAD_BEEF)), 0xDEAD_BEEF);
    assert_eq!(parse_be_u16s(&[0x01, 0xbb, 0x00, 0x50, 0xff]), vec![443, 80]);
    assert!(parse_be_u16s(&[]).is_empty());
}
