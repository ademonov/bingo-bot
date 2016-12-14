use super::*;

#[test]
fn bingo_tests_works() {
    assert_eq!(cards::ROW_COUNT, cards::COL_COUNT);
}

#[test]
fn cardcell_new_tests() {
    let c = cards::CardCell::new("something");

    assert_eq!(false, c.checked); 
    assert_eq!("something", c.word);
    assert_ne!("somethingelse", c.word);
}