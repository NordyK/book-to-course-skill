use super::*;

fn sample() -> (Rc<Node>, Rc<Node>, Rc<Node>) {
    let root = new_node("root");
    let usr = new_node("usr");
    let bin = new_node("bin");
    add_child(&root, &usr);
    add_child(&usr, &bin);
    add_child(&root, &new_node("etc"));
    (root, usr, bin)
}

#[test]
fn parents() {
    let (root, usr, bin) = sample();
    assert_eq!(parent_name(&root), None);
    assert_eq!(parent_name(&usr), Some(String::from("root")));
    assert_eq!(parent_name(&bin), Some(String::from("usr")));
}

#[test]
fn paths() {
    let (root, _usr, bin) = sample();
    assert_eq!(path(&bin), "root/usr/bin");
    assert_eq!(path(&root), "root");
}

#[test]
fn counting() {
    let (root, usr, _) = sample();
    assert_eq!(count(&root), 4);
    assert_eq!(count(&usr), 2);
}

#[test]
fn no_cycle() {
    let (root, usr, _) = sample();
    assert_eq!(Rc::strong_count(&root), 1, "children do not own their parent");
    assert_eq!(Rc::strong_count(&usr), 2, "usr is owned by its variable and by root");
    assert_eq!(Rc::weak_count(&root), 2, "two children point back weakly");
}

#[test]
fn parent_can_be_dropped() {
    let bin = {
        let (_root, _usr, bin) = sample();
        bin
    };
    assert_eq!(parent_name(&bin), None, "the parents were freed");
    assert_eq!(path(&bin), "bin");
}
