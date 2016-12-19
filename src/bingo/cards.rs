use std::iter::Iterator;
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

pub struct Card<'a> {
    created: Tm,
    cells: [[CardCell<'a>; COL_COUNT]; ROW_COUNT]
}

impl<'a> Card<'a> {
    pub fn new<I>(word_iter: I) -> Card<'a> where I: Iterator<Item=&'a str> {
        Card { created: now_utc(), cells: fill_cells(word_iter) }
    }
}

fn fill_cells<'a, I>(word_iter: I) -> [[CardCell<'a>; COL_COUNT]; ROW_COUNT] where I: Iterator<Item=&'a str> {
    let mut result = [[CardCell::new("(none)"); COL_COUNT]; ROW_COUNT];
    let mut word_vec = word_iter.collect::<Vec<_>>();

    for row in 0..ROW_COUNT {
        for col in 0..COL_COUNT {
            let vector_index = (word_vec.len() - 1) / 2; // use random here
            let word = word_vec.remove(vector_index);
            result[row][col] = CardCell::new(word);
        }
    }

    result
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

impl<'a> Format for Card<'a> {
    fn format(&self, fp: &FormatProvider) -> String {

        // for row in 0..ROW_COUNT {
        //     for col in 0..COL_COUNT {
        //         self.cells[row][col]
        //     }
        // }

        unimplemented!();
    }
}
