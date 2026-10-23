use super::*;

fn pair(k: &str, v: &str) -> Option<(String, String)> {
    Some((k.to_string(), v.to_string()))
}

#[test]
fn parses_settings() {
    assert_eq!(parse_line(" name = demo "), pair("name", "demo"));
    assert_eq!(parse_line("url=http://x/?a=b"), pair("url", "http://x/?a=b"), "only the first = splits");
    assert_eq!(parse_line("empty ="), pair("empty", ""), "an empty value is allowed");
}

#[test]
fn skips_non_settings() {
    assert_eq!(parse_line(""), None, "blank");
    assert_eq!(parse_line("   "), None, "spaces only");
    assert_eq!(parse_line("# port = 1"), None, "comment");
    assert_eq!(parse_line("no equals sign"), None, "no =");
    assert_eq!(parse_line(" = value"), None, "empty key");
}

#[test]
fn finds_the_port() {
    assert_eq!(port(&["debug = true", "port = 3000"]), 3000);
    assert_eq!(port(&["debug = true"]), 8080, "missing");
    assert_eq!(port(&["port = lots"]), 8080, "not a number");
    assert_eq!(port(&["port = 70000"]), 8080, "too big for u16");
    assert_eq!(port(&["# port = 1", "port=22"]), 22, "comments are skipped");
}

#[test]
fn counts() {
    assert_eq!(count_settings(&["a=1", "# b=2", "", "c = 3", "junk"]), 2);
}
