/// Returns the words from the string
pub fn split<'a, I>(s: &'a str) -> I where I: Iterator<Item = &'a &'a str> {
    unimplemented!()
}

pub struct Word {
    title: String,
    synonyms: Vec<String>
}

impl Word {
     pub fn new(title: &str, synonyms: Vec<&str>) -> Word {
         Word { 
             title: title.to_owned(), 
             synonyms: synonyms.iter().map(|&s|s.to_owned()).collect::<Vec<_>>() }
     }
}