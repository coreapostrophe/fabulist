use crate::token::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
        }
    }
}
