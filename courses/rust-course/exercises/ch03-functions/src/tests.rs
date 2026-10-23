use super::*;

#[test]
fn squares() {
    assert_eq!(square(-7), 49, "square(-7)");
    assert_eq!(square(0), 0, "square(0)");
    assert_eq!(square(100_000), 10_000_000_000, "square(100_000) needs i64");
}

#[test]
fn hypotenuses() {
    assert!((hypotenuse(3.0, 4.0) - 5.0).abs() < 1e-9, "hypotenuse(3.0, 4.0) should be 5.0");
    assert!((hypotenuse(1.0, 1.0) - 2f64.sqrt()).abs() < 1e-9, "hypotenuse(1.0, 1.0) should be √2");
}

#[test]
fn clamping() {
    assert_eq!(clamp_score(130), 100, "clamp_score(130)");
    assert_eq!(clamp_score(-5), 0, "clamp_score(-5)");
    assert_eq!(clamp_score(64), 64, "clamp_score(64)");
}

#[test]
fn grades_at_the_edges() {
    assert_eq!(grade(90), 'A', "grade(90)");
    assert_eq!(grade(89), 'B', "grade(89)");
    assert_eq!(grade(80), 'B', "grade(80)");
    assert_eq!(grade(79), 'C', "grade(79)");
    assert_eq!(grade(70), 'C', "grade(70)");
    assert_eq!(grade(69), 'F', "grade(69)");
}

#[test]
fn grades_out_of_range() {
    assert_eq!(grade(250), 'A', "grade(250) is clamped to 100");
    assert_eq!(grade(-3), 'F', "grade(-3) is clamped to 0");
}
