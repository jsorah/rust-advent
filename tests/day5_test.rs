use advent::year::year2015::day5::{Day5, NaughtyOrNice};

#[test]
fn nice_string_2() {
    let value = String::from("ugknbfddgicrmopn");
    let result = NaughtyOrNice::Nice(value.clone());
    assert_eq!(Day5::nice_string(value.clone()), result);
}

#[test]
fn nice_string_3() {
    let value = String::from("aaa");
    let result = NaughtyOrNice::Nice(value.clone());
    assert_eq!(Day5::nice_string(value.clone()), result);
}

#[test]
fn should_be_naughty_because_no_double_letter() {
    let value = String::from("jchzalrnumimnmhp");
    let result = NaughtyOrNice::Naughty(value.clone());
    assert_eq!(Day5::nice_string(value.clone()), result);
}
#[test]

fn should_be_naughty_due_to_xy() {
    let value = String::from("haegwjzuvuyypxyu");
    let result = NaughtyOrNice::Naughty(value.clone());
    assert_eq!(Day5::nice_string(value.clone()), result);
}
#[test]

fn should_be_naughty_because_one_vowel() {
    let value = String::from("dvszwmarrgswjxmb");
    let result = NaughtyOrNice::Naughty(value.clone());
    assert_eq!(Day5::nice_string(value.clone()), result);
}