#[macro_use]
extern crate log;
extern crate log4rs;

mod logger; 


fn main() {
    logger::init();
    info!("Starting...");
    

} 
