pub const CARD_ROW_COUNT: usize = 5;
pub const CARD_COL_COUNT: usize = 5;

pub struct User<'a> {
    pub username: String,
    pub card: Option<Card<'a>>
}

pub struct Cell<'a> {
    pub word: &'a str,
    pub checked: bool
}

pub struct Card<'a> {
    cells: [[Cell<'a>; CARD_COL_COUNT]; CARD_ROW_COUNT]
}