use std::fmt::Display;

use crate::keywords::KeywordKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Asterisk,
    Slash,
    Colon,
    Semicolon,
    Pound,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    ArrowRight,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    Keyword(KeywordKind),

    EoF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::Pound => write!(f, "#"),
            Token::Bang => write!(f, "!"),
            Token::BangEqual => write!(f, "!="),
            Token::Equal => write!(f, "="),
            Token::EqualEqual => write!(f, "=="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::ArrowRight => write!(f, "=>"),
            Token::Identifier(name) => write!(f, "identifier `{}`", name),
            Token::String(value) => write!(f, "string `{}`", value),
            Token::Number(value) => write!(f, "number `{}`", value),
            Token::Keyword(kind) => write!(f, "keyword `{}`", kind),
            Token::EoF => write!(f, "EOF"),
        }
    }
}
