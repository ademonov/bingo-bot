#[macro_use]
extern crate log;
extern crate log4rs;
extern crate hyper;

mod logger; 
mod http;

fn main() {
    logger::init();
    info!("Starting...");
    
    http::listening("0.0.0.0:8181");

    info!("Done.")
} 
