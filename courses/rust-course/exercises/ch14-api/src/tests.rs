use super::*;
use crate::{mix, PrimaryColor, SecondaryColor};

#[test]
fn mixes_pairs() {
    assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Yellow), Some(SecondaryColor::Orange));
    assert_eq!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), Some(SecondaryColor::Green));
    assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Blue), Some(SecondaryColor::Purple));
}

#[test]
fn order_does_not_matter() {
    assert_eq!(mix(PrimaryColor::Blue, PrimaryColor::Red), Some(SecondaryColor::Purple));
    assert_eq!(mix(PrimaryColor::Yellow, PrimaryColor::Red), Some(SecondaryColor::Orange));
}

#[test]
fn same_color_is_none() {
    assert_eq!(mix(PrimaryColor::Blue, PrimaryColor::Blue), None);
}

#[test]
fn old_paths_still_work() {
    assert_eq!(crate::utils::mix(crate::kinds::PrimaryColor::Red, crate::kinds::PrimaryColor::Yellow), Some(crate::kinds::SecondaryColor::Orange));
}
