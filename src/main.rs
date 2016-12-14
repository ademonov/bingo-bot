#[macro_use]
extern crate log;
extern crate log4rs;
extern crate hyper;
extern crate time;

mod logger; 
mod http;
mod bingo;

fn main() {
    logger::init();
    info!("Starting...");
    info!("card-size: {}x{}", bingo::cards::ROW_COUNT, bingo::cards::COL_COUNT);
    http::listening("0.0.0.0:8181");

    info!("Done.")
} 
