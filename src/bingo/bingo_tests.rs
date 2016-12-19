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
fn card_new() {
    let c = cards::Card::new();
}
