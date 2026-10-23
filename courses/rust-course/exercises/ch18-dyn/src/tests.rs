use super::*;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

fn sample() -> Registry {
    let mut r = Registry::new();
    r.add(Box::new(Circle { r: 1.0 }));
    r.add(Box::new(Rect { w: 2.0, h: 3.0 }));
    r.add(Box::new(Rect { w: 1.0, h: 1.0 }));
    r
}

#[test]
fn areas() {
    let r = sample();
    assert!(close(r.total_area(), std::f64::consts::PI + 7.0));
}

#[test]
fn largest_shape() {
    let r = sample();
    let big = r.largest().unwrap();
    assert_eq!(big.name(), "rect");
    assert!(close(big.area(), 6.0));
    assert!(format!("{big:?}").contains("Rect"), "Debug via the supertrait");
    assert!(Registry::new().largest().is_none());
}

#[test]
fn scaling() {
    let mut r = sample();
    r.scale_all(2.0);
    assert!(close(r.total_area(), 4.0 * (std::f64::consts::PI + 7.0)));
}

#[test]
fn counting() {
    let r = sample();
    assert_eq!(r.count_by_name("rect"), 2);
    assert_eq!(r.count_by_name("circle"), 1);
    assert_eq!(r.count_by_name("hexagon"), 0);
}

#[test]
fn generic_helper() {
    let circles = [Circle { r: 1.0 }, Circle { r: 2.0 }];
    assert!(close(sum_areas(&circles), 5.0 * std::f64::consts::PI));
}
