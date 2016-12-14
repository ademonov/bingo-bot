pub mod cards;

use time::Tm;

pub struct CardCell<'a> {
    pub word: &'a str,
    pub checked: bool
}



pub struct User<'a> {
    pub username: String,
    pub card: Option<Card<'a>>
}

pub struct Card<'a> {
    created: Tm,
    cells: [[CardCell<'a>; cards::COL_COUNT]; cards::ROW_COUNT]
}