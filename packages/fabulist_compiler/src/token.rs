use std::fmt::Display;

#[derive(Debug)]
pub enum TokenType {
    // Literals
    IDENTIFIER,

    // One or two character tokens,
    EQUAL,
    EQUAL_EQUAL,

    // KEYWORDS
    RETURN,
    IF,

    // Single-character
    LEFT_BRACKET,
    RIGHT_BRACKET,
    LEFT_BRACE,
    RIGHT_BRACE,
    COLON,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("line:{} {} {}", self.line, self.token_type, self.lexeme)
    }
}
