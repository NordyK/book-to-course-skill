use super::*;

#[test]
fn frequencies() {
    let f = word_frequencies("The cat the hat");
    assert_eq!(f.get("the"), Some(&2));
    assert_eq!(f.get("cat"), Some(&1));
    assert_eq!(f.len(), 3);
    assert!(word_frequencies("").is_empty());
}

#[test]
fn adding() {
    let mut dir = BTreeMap::new();
    assert!(add_employee(&mut dir, "Add Sally to Engineering"));
    assert!(add_employee(&mut dir, "Add Amir to Sales"));
    assert!(add_employee(&mut dir, "Add Bea to Engineering"));
    assert_eq!(dir["Engineering"], vec!["Sally", "Bea"]);
    assert_eq!(dir.len(), 2);
}

#[test]
fn bad_commands() {
    let mut dir = BTreeMap::new();
    assert!(!add_employee(&mut dir, "Remove Sally from Engineering"));
    assert!(!add_employee(&mut dir, "Add Sally"));
    assert!(!add_employee(&mut dir, ""));
    assert!(dir.is_empty(), "nothing was added");
}

#[test]
fn listing() {
    let mut dir = BTreeMap::new();
    add_employee(&mut dir, "Add Zoe to Ops");
    add_employee(&mut dir, "Add Ann to Ops");
    assert_eq!(department_list(&dir, "Ops"), vec!["Ann", "Zoe"]);
    assert!(department_list(&dir, "Legal").is_empty());
}

#[test]
fn letters() {
    assert_eq!(unique_letters("banana"), 3);
    assert_eq!(unique_letters(""), 0);
    assert_eq!(unique_letters("ééa"), 2);
}
