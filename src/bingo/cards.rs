use time::{Tm, now_utc};

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

     pub fn GetWord(&self) -> &'a str {
         self.word
     }

     pub fn IsChecked(&self) -> bool {
         self.checked
     }

     pub fn Check(&mut self) {
         self.checked = true;
     }
}

pub struct Card<'a> {
    created: Tm,
    cells: [[CardCell<'a>; COL_COUNT]; ROW_COUNT]
}

fn fill_cells<'a>() -> [[CardCell<'a>; COL_COUNT]; ROW_COUNT] {
    let mut result = [[CardCell::new("0x0"); COL_COUNT]; ROW_COUNT];
    
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