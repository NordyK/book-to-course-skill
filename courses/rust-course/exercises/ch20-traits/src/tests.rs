use super::*;

fn v(x: f64, y: f64) -> Vec2 {
    Vec2 { x, y }
}

#[test]
fn arithmetic() {
    assert_eq!(v(1.0, 2.0) + v(3.0, 4.0), v(4.0, 6.0));
    assert_eq!(v(1.0, 2.0) - v(3.0, 5.0), v(-2.0, -3.0));
    assert_eq!(v(1.0, 2.0) * 3.0, v(3.0, 6.0));
    assert_eq!(v(1.0, 2.0) * v(3.0, 4.0), 11.0);
    assert_eq!(-v(1.0, -2.0), v(-1.0, 2.0));
    assert_eq!(v(3.0, 4.0).length(), 5.0);
}

#[test]
fn assign_and_index() {
    let mut a = v(1.0, 1.0);
    a += v(2.0, 3.0);
    assert_eq!(a, v(3.0, 4.0));
    assert_eq!(a[0], 3.0);
    a[1] = 10.0;
    assert_eq!(a.y, 10.0);
}

#[test]
#[should_panic]
fn bad_index() {
    let a = v(0.0, 0.0);
    let _ = a[2];
}

#[test]
fn names() {
    assert_eq!(both_names(&v(3.0, 4.0)), (String::from("vector"), String::from("(3, 4)")));
    assert_eq!(both_names(&v(0.5, -1.0)).1, "(0.5, -1)");
}
