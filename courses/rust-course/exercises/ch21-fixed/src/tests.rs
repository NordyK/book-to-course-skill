use super::*;

fn q(x: f64) -> Q8_8 {
    Q8_8::from(x)
}

#[test]
fn conversions() {
    assert_eq!(q(1.5), Q8_8(384));
    assert_eq!(q(-0.25), Q8_8(-64));
    assert_eq!(q(0.001953125), Q8_8(1), "rounds half up to one step");
    assert_eq!(q(0.0019), Q8_8(0), "rounds down below half a step");
    assert_eq!(q(500.0), Q8_8(i16::MAX));
    assert_eq!(q(-500.0), Q8_8(i16::MIN));
    assert_eq!(f64::from(Q8_8(384)), 1.5);
    assert_eq!(f64::from(Q8_8(i16::MIN)), -128.0);
}

#[test]
fn arithmetic() {
    assert_eq!(q(1.5) + q(2.25), q(3.75));
    assert_eq!(q(1.0) - q(3.5), q(-2.5));
    assert_eq!(q(2.0) * q(-1.25), q(-2.5));
    assert_eq!(q(0.5) * q(0.5), q(0.25));
    assert_eq!(-q(3.0), q(-3.0));
}

#[test]
fn saturation() {
    assert_eq!(q(100.0) + q(100.0), Q8_8(i16::MAX));
    assert_eq!(q(-100.0) - q(100.0), Q8_8(i16::MIN));
    assert_eq!(q(20.0) * q(20.0), Q8_8(i16::MAX));
    assert_eq!(-Q8_8(i16::MIN), Q8_8(i16::MAX));
}

#[test]
fn display() {
    assert_eq!(q(1.5).to_string(), "1.5000");
    assert_eq!(q(-0.25).to_string(), "-0.2500");
    assert_eq!(Q8_8(1).to_string(), "0.0039");
}

#[test]
fn random() {
    assert_eq!(rand_unit(0), 0.0);
    assert_eq!(rand_unit(0x80), 0.5);
    assert_eq!(rand_unit(0xff), 0.99609375);
    for b in 0..=255u8 {
        assert_eq!(rand_unit(b), b as f32 / 256.0);
    }
}
