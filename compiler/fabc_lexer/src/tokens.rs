use std::fmt::Display;

use crate::keywords::KeywordKind;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    UnrecognizedCharacter,
    UnterminatedString,
    InvalidNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind<'src> {
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
    Commat,

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
    Identifier(&'src str),
    String(&'src str),
    Number(f64),
    Keyword(KeywordKind),

    Error(ErrorKind),
    EoF,
}

impl<'src> Display for TokenKind<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Commat => write!(f, "@"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Pound => write!(f, "#"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::BangEqual => write!(f, "!="),
            TokenKind::Equal => write!(f, "="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::ArrowRight => write!(f, "=>"),
            TokenKind::Identifier(name) => write!(f, "identifier `{}`", name),
            TokenKind::String(value) => write!(f, "string `{}`", value),
            TokenKind::Number(value) => write!(f, "number `{}`", value),
            TokenKind::Keyword(kind) => write!(f, "keyword `{}`", kind),
            TokenKind::Error(kind) => write!(f, "error `{:?}`", kind),
            TokenKind::EoF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}
