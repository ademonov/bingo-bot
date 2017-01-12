use std::io::prelude::*;
use std::fs::File;
use std::io::Result;

pub fn load_words() {
    let s = load_content("1.yml").unwrap();
    println!("{}", s);
}

fn load_content(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;    
    Ok(result)
}