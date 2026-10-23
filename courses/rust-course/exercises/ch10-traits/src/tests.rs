use super::*;

fn p(name: &str, points: u32) -> Player {
    Player { name: name.to_string(), points }
}

#[test]
fn player_uses_default_label() {
    assert_eq!(p("Ada", 7).label(), "Ada: 7 pts");
}

#[test]
fn team_overrides_label() {
    let t = Team { name: String::from("Red"), members: vec![p("a", 5), p("b", 7)] };
    assert_eq!(t.points(), 12);
    assert_eq!(t.label(), "Team Red: 12 pts (2 members)");
}

#[test]
fn best_of() {
    let players = [p("a", 3), p("b", 9), p("c", 9)];
    assert_eq!(best(&players).unwrap().name(), "b", "first of equal scores");
    let none: [Player; 0] = [];
    assert!(best(&none).is_none());
    let teams = [Team { name: "X".into(), members: vec![p("a", 1)] }, Team { name: "Y".into(), members: vec![p("b", 2)] }];
    assert_eq!(best(&teams).unwrap().name(), "Y", "works for any Scored type");
}

#[test]
fn reports() {
    assert_eq!(report(&p("Bo", 0)), "[Bo: 0 pts]");
    let t = Team { name: String::from("Empty"), members: vec![] };
    assert_eq!(report(&t), "[Team Empty: 0 pts (0 members)]");
}
