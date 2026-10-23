use super::*;

#[test]
fn leap_years() {
    assert!(is_leap(2024) && is_leap(2000) && !is_leap(1900) && !is_leap(2023));
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(days_in_month(2023, 2), 28);
    assert_eq!(days_in_month(2023, 4), 30);
    assert_eq!(days_in_month(2023, 12), 31);
}

#[test]
fn civil_roundtrip() {
    assert_eq!(days_from_civil(1970, 1, 1), 0);
    assert_eq!(days_from_civil(2000, 3, 1), 11_017);
    assert_eq!(civil_from_days(-1), (1969, 12, 31));
    for z in (-800_000..800_000).step_by(997) {
        let (y, m, d) = civil_from_days(z);
        assert_eq!(days_from_civil(y, m, d), z);
    }
}

#[test]
fn formatting() {
    assert_eq!(to_rfc3339(0), "1970-01-01T00:00:00Z");
    assert_eq!(to_rfc3339(1_709_211_909), "2024-02-29T13:05:09Z");
    assert_eq!(to_rfc3339(-86_401), "1969-12-30T23:59:59Z");
}

#[test]
fn parsing() {
    assert_eq!(parse_rfc3339("2024-02-29T13:05:09Z"), Some(1_709_211_909));
    assert_eq!(parse_rfc3339("1970-01-01T00:00:00Z"), Some(0));
    for bad in [
        "2023-02-29T00:00:00Z",
        "2024-13-01T00:00:00Z",
        "2024-04-31T00:00:00Z",
        "2024-01-01T24:00:00Z",
        "2024-01-01T00:60:00Z",
        "2024-01-01 00:00:00Z",
        "2024-01-01T00:00:00",
        "24-01-01T00:00:00Z",
        "2024-0a-01T00:00:00Z",
    ] {
        assert_eq!(parse_rfc3339(bad), None, "{bad}");
    }
    for t in [0i64, 951_782_400, 4_102_444_799, -1_000_000] {
        assert_eq!(parse_rfc3339(&to_rfc3339(t)), Some(t));
    }
}

#[test]
fn weekdays() {
    assert_eq!(weekday(0), "Thu");
    assert_eq!(weekday(1_709_211_909), "Thu");
    assert_eq!(weekday(1_709_251_200), "Fri");
    assert_eq!(weekday(-86_400), "Wed");
}

#[test]
fn ntp() {
    assert_eq!(ntp_to_unix(2_208_988_800, 0), (0, 0));
    assert_eq!(ntp_to_unix(3_918_200_709, 1 << 31), (1_709_211_909, 500_000_000));
    assert_eq!(unix_to_ntp(1_709_211_909, 500_000_000), (3_918_200_709, 1 << 31));
    let (s, f) = unix_to_ntp(100, 123_456_789);
    let (s2, n2) = ntp_to_unix(s, f);
    assert_eq!(s2, 100);
    assert!((n2 as i64 - 123_456_789).abs() <= 1, "fraction round-trips to within 1 ns");
}

#[test]
fn offsets() {
    assert_eq!(ntp_offset_delay(1000.0, 1110.0, 1111.0, 1021.0), (100.0, 20.0));
    let samples = [
        (0.0, 60.0, 61.0, 21.0),    // offset 50, delay 20
        (0.0, 210.0, 211.0, 201.0), // offset 110, delay 200
        (0.0, 5.0, 5.0, 0.0),       // delay 0: ignored
    ];
    let best = best_offset(&samples).unwrap();
    let expected = (50.0 / 400.0 + 110.0 / 40_000.0) / (1.0 / 400.0 + 1.0 / 40_000.0);
    assert!((best - expected).abs() < 1e-9);
    assert_eq!(best_offset(&[(0.0, 5.0, 5.0, 0.0)]), None);
    assert_eq!(best_offset(&[]), None);
}
