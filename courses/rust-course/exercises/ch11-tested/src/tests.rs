use super::*;

#[test]
fn clamps() {
    assert_eq!(clamp_percent(-5), 0);
    assert_eq!(clamp_percent(42), 42);
    assert_eq!(clamp_percent(150), 100);
}

#[test]
fn converts_to_celsius() {
    let t = Temperature::from_kelvin(273.15);
    assert!((t.celsius() - 0.0).abs() < 1e-9, "273.15 K is 0 °C, got {}", t.celsius());
    assert_eq!(Temperature::from_kelvin(0.0).celsius(), -273.15);
}

#[test]
#[should_panic(expected = "below absolute zero")]
fn rejects_negative_kelvin() {
    Temperature::from_kelvin(-1.0);
}

#[test]
fn parses_pairs() -> Result<(), String> {
    let (k, v) = parse_kv(" depth = 3 ")?;
    assert_eq!(k, "depth");
    assert_eq!(v, 3);
    let (k, v) = parse_kv("x=-7")?;
    assert_eq!((k.as_str(), v), ("x", -7));
    Ok(())
}

#[test]
fn reports_errors() {
    assert_eq!(parse_kv("depth 3"), Err(String::from("missing =")));
    assert_eq!(parse_kv("depth=three"), Err(String::from("bad number")));
    assert_eq!(parse_kv(" = 3"), Err(String::from("empty key")));
}
