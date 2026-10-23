use super::*;

const POEM: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

fn args(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[test]
fn builds_config() {
    let c = Config::build(&args(&["mg", "to", "poem.txt"]), false).unwrap();
    assert_eq!(c, Config { query: "to".into(), file_path: "poem.txt".into(), ignore_case: false });
    assert!(Config::build(&args(&["mg", "to", "poem.txt"]), true).unwrap().ignore_case, "env var respected");
}

#[test]
fn options_override_env() {
    assert!(Config::build(&args(&["mg", "to", "p", "-i"]), false).unwrap().ignore_case);
    assert!(!Config::build(&args(&["mg", "to", "p", "-s"]), true).unwrap().ignore_case);
}

#[test]
fn build_errors() {
    assert_eq!(Config::build(&args(&["mg", "to"]), false), Err("not enough arguments"));
    assert_eq!(Config::build(&args(&["mg", "to", "p", "-x"]), false), Err("unknown option"));
}

#[test]
fn case_sensitive() {
    let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
    assert_eq!(search("duct", contents), vec!["safe, fast, productive."]);
    assert_eq!(search("frog", POEM), vec!["How public, like a frog"]);
}

#[test]
fn case_insensitive() {
    let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
    assert_eq!(search_case_insensitive("rUsT", contents), vec!["Rust:", "Trust me."]);
}

#[test]
fn run_writes_matches() {
    let mut out: Vec<u8> = Vec::new();
    let c = Config { query: "to".into(), file_path: "poem.txt".into(), ignore_case: true };
    let n = run_on(&c, POEM, &mut out).unwrap();
    assert_eq!(n, 4);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        "Are you nobody, too?\nHow dreary to be somebody!\nTo tell your name the livelong day\nTo an admiring bog!\n"
    );
}

#[test]
fn run_with_no_matches() {
    let mut out: Vec<u8> = Vec::new();
    let c = Config { query: "monomorphization".into(), file_path: "p".into(), ignore_case: false };
    assert_eq!(run_on(&c, POEM, &mut out).unwrap(), 0);
    assert!(out.is_empty());
}
