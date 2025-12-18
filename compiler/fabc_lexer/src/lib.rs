use crate::{error::Error, keywords::KeywordKind, tokens::Token};

pub mod error;
pub mod keywords;
pub mod tokens;

pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<&Vec<Token>, Error> {
        self.scan_tokens()?;
        Ok(&self.tokens)
    }

    fn scan_tokens(&mut self) -> Result<(), Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::EoF);

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance()?;

        match c {
            // Single-character tokens.
            '(' => self.tokens.push(Token::LeftParen),
            ')' => self.tokens.push(Token::RightParen),
            '{' => self.tokens.push(Token::LeftBrace),
            '}' => self.tokens.push(Token::RightBrace),
            ',' => self.tokens.push(Token::Comma),
            '.' => self.tokens.push(Token::Dot),
            '-' => self.tokens.push(Token::Minus),
            '+' => self.tokens.push(Token::Plus),
            '*' => self.tokens.push(Token::Asterisk),
            ';' => self.tokens.push(Token::Semicolon),

            // Double-character tokens.
            '!' => {
                if self.r#match('=')? {
                    self.tokens.push(Token::BangEqual)
                } else {
                    self.tokens.push(Token::Bang)
                }
            }
            '=' => {
                if self.r#match('=')? {
                    self.tokens.push(Token::EqualEqual)
                } else {
                    self.tokens.push(Token::Equal)
                }
            }
            '<' => {
                if self.r#match('=')? {
                    self.tokens.push(Token::LessEqual)
                } else {
                    self.tokens.push(Token::Less)
                }
            }
            '>' => {
                if self.r#match('=')? {
                    self.tokens.push(Token::GreaterEqual)
                } else {
                    self.tokens.push(Token::Greater)
                }
            }

            // Comments and whitespace.
            '/' => {
                if self.r#match('/')? {
                    while self.peek()? != '\n' && !self.is_at_end() {
                        self.advance()?;
                    }
                } else {
                    self.tokens.push(Token::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }

            // Literals.
            '"' => {
                self.string()?;
            }
            '0'..='9' => {
                self.number()?;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                self.identifier()?;
            }
            _ => return Err(Error::UnexpectedCharacter(c)),
        }

        Ok(())
    }

    fn identifier(&mut self) -> Result<(), Error> {
        while self.peek()?.is_ascii_alphanumeric() || self.peek()? == '_' {
            self.advance()?;
        }

        if self.peek()? == ':' && self.peek_next()? == ':' {
            return self.path();
        }

        let text = &self.source[self.start..self.current];
        let keyword_kind = KeywordKind::get(text);

        if let Some(keyword_kind) = keyword_kind {
            self.tokens.push(Token::Keyword(keyword_kind));
        } else {
            self.tokens.push(Token::Identifier(text.to_string()));
        }

        Ok(())
    }

    fn path(&mut self) -> Result<(), Error> {
        let mut segments = Vec::new();

        loop {
            while self.peek()?.is_ascii_alphanumeric() || self.peek()? == '_' {
                self.advance()?;
            }

            let segment = &self.source[self.start..self.current];
            segments.push(segment.to_string());

            if self.peek()? != ':' {
                break;
            }

            self.advance()?;
            self.advance()?;

            self.start = self.current;
        }

        self.tokens.push(Token::Path(segments));

        Ok(())
    }

    fn number(&mut self) -> Result<(), Error> {
        while self.peek()?.is_ascii_digit() {
            self.advance()?;
        }

        if self.peek()? == '.' && self.peek_next()?.is_ascii_digit() {
            self.advance()?;

            while self.peek()?.is_ascii_digit() {
                self.advance()?;
            }
        }

        let number_str = &self.source[self.start..self.current];
        self.tokens.push(Token::Number(
            number_str.parse().map_err(|_| Error::UnableToParseNumber)?,
        ));

        Ok(())
    }

    fn peek_next(&self) -> Result<char, Error> {
        if self.current + 1 >= self.source.len() {
            return Ok('\0');
        }
        self.source
            .chars()
            .nth(self.current + 1)
            .ok_or(Error::UnexpectedEndOfInput)
    }

    fn string(&mut self) -> Result<(), Error> {
        while self.peek()? != '"' && !self.is_at_end() {
            if self.peek()? == '\n' {
                self.line += 1;
            }
            self.advance()?;
        }

        if self.is_at_end() {
            return Err(Error::UnterminatedString);
        }

        self.advance()?;

        let value = &self.source[self.start + 1..self.current - 1];
        self.tokens.push(Token::String(value.to_string()));

        Ok(())
    }

    fn r#match(&mut self, expected: char) -> Result<bool, Error> {
        if self.is_at_end() {
            return Ok(false);
        }
        if self
            .source
            .chars()
            .nth(self.current)
            .ok_or(Error::UnexpectedEndOfInput)?
            != expected
        {
            return Ok(false);
        }

        self.current += 1;
        Ok(true)
    }

    fn peek(&self) -> Result<char, Error> {
        if self.is_at_end() {
            return Ok('\0');
        }
        self.source
            .chars()
            .nth(self.current)
            .ok_or(Error::UnexpectedEndOfInput)
    }

    fn advance(&mut self) -> Result<char, Error> {
        let ch = self
            .source
            .chars()
            .nth(self.current)
            .ok_or(Error::UnexpectedEndOfInput)?;
        self.current += 1;
        Ok(ch)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let source = "( ) { } , . - + * ; ! != = == < <= > >= /";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Dot,
            Token::Minus,
            Token::Plus,
            Token::Asterisk,
            Token::Semicolon,
            Token::Bang,
            Token::BangEqual,
            Token::Equal,
            Token::EqualEqual,
            Token::Less,
            Token::LessEqual,
            Token::Greater,
            Token::GreaterEqual,
            Token::Slash,
            Token::EoF,
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let source =
            "let fn if else return goto true false none while for and or context myVar path::to::value";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Keyword(KeywordKind::Let),
            Token::Keyword(KeywordKind::Fn),
            Token::Keyword(KeywordKind::If),
            Token::Keyword(KeywordKind::Else),
            Token::Keyword(KeywordKind::Return),
            Token::Keyword(KeywordKind::Goto),
            Token::Keyword(KeywordKind::True),
            Token::Keyword(KeywordKind::False),
            Token::Keyword(KeywordKind::None),
            Token::Keyword(KeywordKind::While),
            Token::Keyword(KeywordKind::For),
            Token::Keyword(KeywordKind::And),
            Token::Keyword(KeywordKind::Or),
            Token::Keyword(KeywordKind::Context),
            Token::Identifier("myVar".to_string()),
            Token::Path(vec![
                "path".to_string(),
                "to".to_string(),
                "value".to_string(),
            ]),
            Token::EoF,
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_string_and_number_literals() {
        let source = r#""hello" 123 45.67"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::String("hello".to_string()),
            Token::Number(123.0),
            Token::Number(45.67),
            Token::EoF,
        ];
        assert_eq!(*tokens, expected_tokens);
    }
}
