use super::*;

#[test]
fn checked_add_fits() {
    assert_eq!(add_checked(200, 55), Some(255), "200 + 55");
    assert_eq!(add_checked(0, 0), Some(0), "0 + 0");
}

#[test]
fn checked_add_overflows() {
    assert_eq!(add_checked(200, 56), None, "200 + 56 is 256, too big for u8");
    assert_eq!(add_checked(255, 255), None, "255 + 255");
}

#[test]
fn volume_stops_at_255() {
    assert_eq!(volume_up(250, 10), 255, "volume_up(250, 10)");
    assert_eq!(volume_up(10, 5), 15, "volume_up(10, 5)");
    assert_eq!(volume_up(255, 1), 255, "volume_up(255, 1)");
}

#[test]
fn odometer_rolls_over() {
    assert_eq!(odometer(65_530, 10), 4, "odometer(65_530, 10)");
    assert_eq!(odometer(100, 23), 123, "odometer(100, 23)");
    assert_eq!(odometer(65_535, 1), 0, "odometer(65_535, 1)");
}
