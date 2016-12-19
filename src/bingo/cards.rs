use time::{Tm, now_utc};
use formatting::{Format, FormatProvider};

pub const ROW_COUNT: usize = 5;
pub const COL_COUNT: usize = 5;

#[derive(Copy, Clone)]
pub struct CardCell<'a> {
    word: &'a str,
    checked: bool
}

impl<'a> CardCell<'a> {
     pub fn new(word: &'a str) -> CardCell<'a> {
         CardCell { word: word, checked: false }
     } 

     pub fn get_word(&self) -> &'a str {
         self.word
     }

     pub fn is_checked(&self) -> bool {
         self.checked
     }

     pub fn check(&mut self) {
         self.checked = true;
     }
}

impl<'a> Format for CardCell<'a> {
    fn format(&self, fp: &FormatProvider) -> String {
        let word = self.get_word();

        if self.is_checked() {
            fp.get_strikethrough(word)
        } else {
            String::from(word)
        }
    }
}

pub struct Card<'a> {
    created: Tm,
    cells: [[CardCell<'a>; COL_COUNT]; ROW_COUNT]
}

fn fill_cells<'a>() -> [[CardCell<'a>; COL_COUNT]; ROW_COUNT] {
    let mut result = [[CardCell::new("(none)"); COL_COUNT]; ROW_COUNT];
    
    /*
    for row in 0..ROW_COUNT {
        for col in 0..COL_COUNT {
            result[row][col] = CardCell::new("0x0");
        }
    }
    */
    
    result
}

impl<'a> Card<'a> {
    pub fn new() -> Card<'a> {
        Card { created: now_utc(), cells: fill_cells() }
    }
}