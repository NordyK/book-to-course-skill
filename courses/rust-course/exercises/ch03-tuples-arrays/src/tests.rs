use super::*;

#[test]
fn min_max_mixed() {
    assert_eq!(min_max([3, -2, 7, 7, 0, 1, 5]), (-2, 7), "min_max([3, -2, 7, 7, 0, 1, 5])");
}

#[test]
fn min_max_all_same() {
    assert_eq!(min_max([4; 7]), (4, 4), "min_max of seven 4s");
}

#[test]
fn min_max_all_negative() {
    assert_eq!(min_max([-5, -9, -1, -3, -8, -2, -7]), (-9, -1), "all below zero");
}

#[test]
fn warmest_first_of_ties() {
    assert_eq!(warmest_day([3, -2, 7, 7, 0, 1, 5]), 2, "two days at 7: the first is index 2");
}

#[test]
fn warmest_at_the_ends() {
    assert_eq!(warmest_day([9, 1, 1, 1, 1, 1, 1]), 0, "Monday is warmest");
    assert_eq!(warmest_day([1, 1, 1, 1, 1, 1, 9]), 6, "Sunday is warmest");
}

#[test]
fn swap_pair() {
    assert_eq!(swap((String::from("Oslo"), 4)), (4, String::from("Oslo")), "swap((\"Oslo\", 4))");
}
