use super::*;

#[test]
fn needs_two_approvals() {
    let mut p = Post::new();
    p.add_text("hello");
    assert_eq!(p.status(), "draft");
    p.request_review();
    assert_eq!(p.status(), "pending");
    p.approve();
    assert_eq!(p.status(), "pending", "one approval is not enough");
    assert_eq!(p.content(), "");
    p.approve();
    assert_eq!(p.status(), "published");
    assert_eq!(p.content(), "hello");
}

#[test]
fn reject_resets() {
    let mut p = Post::new();
    p.add_text("a");
    p.request_review();
    p.approve();
    p.reject();
    assert_eq!(p.status(), "draft");
    p.add_text("b");
    p.request_review();
    p.approve();
    assert_eq!(p.status(), "pending", "approvals were reset by reject");
    p.approve();
    assert_eq!(p.content(), "ab");
}

#[test]
fn editing_only_in_draft() {
    let mut p = Post::new();
    p.add_text("x");
    p.request_review();
    p.add_text("ignored");
    p.approve();
    p.approve();
    p.add_text("also ignored");
    assert_eq!(p.content(), "x");
}

#[test]
fn invalid_transitions_are_ignored() {
    let mut p = Post::new();
    p.approve();
    p.reject();
    assert_eq!(p.status(), "draft");
}

#[test]
fn type_state_workflow() {
    let mut d = Draft::new();
    d.add_text("typed");
    let pending = d.request_review();
    let mut back = pending.reject();
    back.add_text(" post");
    let published = back.request_review().approve();
    assert_eq!(published.content(), "typed post");
}
