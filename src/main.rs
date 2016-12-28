#[macro_use]
extern crate log;
extern crate log4rs;
extern crate hyper;
extern crate time;

mod logger; 
mod http;
mod formatting;
mod bingo;

use formatting::*;

fn main() {
    logger::init();
    info!("Starting...");

    let words = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25"];
    print_iter(words.iter());

    let fp = formatting::GitterMdFormatProvider::new();

    let mut c = bingo::cards::Card::new(words.iter());
    c.check_cell(1,2);
    let sc = c.format(&fp);
    println!("{}", sc);

    //http::listening("0.0.0.0:8181");
    info!("Done.")
} 

fn print_iter<I, S>(iterator: I) where I: Iterator<Item=S>, S: AsRef<str> {
    for item in iterator {
        print!("{} ", item.as_ref())
    }
    println!();
}
