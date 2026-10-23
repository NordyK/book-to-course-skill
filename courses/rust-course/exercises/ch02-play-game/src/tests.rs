use super::*;

fn strings(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[test]
fn three_guesses_until_a_win() {
    assert_eq!(
        play(61, &["10", "80", "61"]),
        strings(&["You guessed: 10", "Too small!", "You guessed: 80", "Too big!", "You guessed: 61", "You win!"]),
    );
}

#[test]
fn bad_input_gets_a_second_chance() {
    assert_eq!(
        play(5, &["x", "5", "9"]),
        strings(&["Please type a number!", "You guessed: 5", "You win!"]),
    );
}

#[test]
fn lines_after_the_win_are_ignored() {
    assert_eq!(play(3, &["3", "4", "5"]), strings(&["You guessed: 3", "You win!"]));
}

#[test]
fn running_out_of_lines_without_winning() {
    assert_eq!(
        play(50, &["20\n", "", "200"]),
        strings(&["You guessed: 20", "Too small!", "Please type a number!", "Please type a number!"]),
    );
}

#[test]
fn no_lines_no_messages() {
    assert_eq!(play(50, &[]), Vec::<String>::new());
}
