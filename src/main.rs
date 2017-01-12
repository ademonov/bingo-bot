#[macro_use]
extern crate log;
extern crate log4rs;
extern crate hyper;
extern crate time;

mod logger; 
mod http;
mod config;
mod formatting;
mod bingo;

fn main() {
    logger::init();
    info!("Starting...");

    config::load_words();

    // let format_provider = formatting::GitterMdFormatProvider::new();

    // http::listening("0.0.0.0:8181");
    info!("Done.");
} 
