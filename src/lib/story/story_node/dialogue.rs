use self::quote::Quote;
use self::speaker::Speaker;

pub mod quote;
pub mod speaker;

pub struct Dialogue {
    speaker: Speaker,
    quotes: Vec<Quote>,
}

impl Dialogue {
    pub fn new(speaker: Speaker, quotes: Vec<Quote>) -> Self {
        Self { speaker, quotes }
    }
    pub fn speaker(&self) -> &Speaker {
        &self.speaker
    }
    pub fn quotes(&self) -> &Vec<Quote> {
        &self.quotes
    }
}
