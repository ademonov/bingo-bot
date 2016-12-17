use super::*;

#[test]
fn bingo_tests_works() {
    assert_eq!(cards::ROW_COUNT, cards::COL_COUNT);
}

#[test]
fn cardcell_new() {
    let c = cards::CardCell::new("something");

    assert_eq!(false, c.IsChecked()); 
    assert_eq!("something", c.GetWord());
    assert_ne!("somethingelse", c.GetWord());
}

#[test]
fn cardcell_check() {
    let mut c = cards::CardCell::new("something");

    c.Check();

    assert_eq!(true, c.IsChecked()); 
    assert_eq!("something", c.GetWord());
}
