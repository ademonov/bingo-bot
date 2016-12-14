use time::Tm;

pub const ROW_COUNT: usize = 5;
pub const COL_COUNT: usize = 5;

pub struct CardCell<'a> {
    pub word: &'a str,
    pub checked: bool
}

impl<'a> CardCell<'a> {
     pub fn new(word: &'a str) -> CardCell<'a> {
         CardCell { word: word, checked: false }
     } 
}

pub struct Card<'a> {
    created: Tm,
    cells: [[CardCell<'a>; COL_COUNT]; ROW_COUNT]
}