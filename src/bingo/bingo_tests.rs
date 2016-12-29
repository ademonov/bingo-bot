use super::*;

#[test]
fn bingo_tests_works() {
    assert_eq!(cards::ROW_COUNT, cards::COL_COUNT);
}

#[test]
fn cardcell_new() {
    let c = cards::CardCell::new("something");

    assert_eq!(false, c.is_checked());
    assert_eq!("something", c.get_word());
    assert_ne!("somethingelse", c.get_word());
}

#[test]
fn cardcell_check() {
    let mut c = cards::CardCell::new("something");

    c.check();

    assert_eq!(true, c.is_checked());
    assert_eq!("something", c.get_word());
}

#[test]
fn card_check_cell() {
    let words = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25"];
    let mut c = cards::Card::new(words.iter());

    c.check_cell(0, 0);
    c.check_cell(cards::ROW_COUNT - 1, cards::COL_COUNT - 1);
}
