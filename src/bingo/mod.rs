pub mod cards;



#[cfg(test)]
mod bingo_tests;

pub struct User<'a> {
    pub username: String,
    pub card: Option<cards::Card<'a>>
}

