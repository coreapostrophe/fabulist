use crate::{error::Error, keywords::KeywordKind, tokens::Tokens};

pub mod error;
pub mod keywords;
pub mod tokens;

pub struct FabCLexer<'a> {
    source: &'a str,
    tokens: Vec<Tokens>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> FabCLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<&Vec<Tokens>, Error> {
        self.scan_tokens()?;
        Ok(&self.tokens)
    }

    fn scan_tokens(&mut self) -> Result<(), Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Tokens::EoF);

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance()?;

        match c {
            // Single-character tokens.
            '(' => self.tokens.push(Tokens::LeftParen),
            ')' => self.tokens.push(Tokens::RightParen),
            '{' => self.tokens.push(Tokens::LeftBrace),
            '}' => self.tokens.push(Tokens::RightBrace),
            ',' => self.tokens.push(Tokens::Comma),
            '.' => self.tokens.push(Tokens::Dot),
            '-' => self.tokens.push(Tokens::Minus),
            '+' => self.tokens.push(Tokens::Plus),
            '*' => self.tokens.push(Tokens::Star),
            ';' => self.tokens.push(Tokens::Semicolon),

            // Double-character tokens.
            '!' => {
                if self.r#match('=')? {
                    self.tokens.push(Tokens::BangEqual)
                } else {
                    self.tokens.push(Tokens::Bang)
                }
            }
            '=' => {
                if self.r#match('=')? {
                    self.tokens.push(Tokens::EqualEqual)
                } else {
                    self.tokens.push(Tokens::Equal)
                }
            }
            '<' => {
                if self.r#match('=')? {
                    self.tokens.push(Tokens::LessEqual)
                } else {
                    self.tokens.push(Tokens::Less)
                }
            }
            '>' => {
                if self.r#match('=')? {
                    self.tokens.push(Tokens::GreaterEqual)
                } else {
                    self.tokens.push(Tokens::Greater)
                }
            }

            // Comments and whitespace.
            '/' => {
                if self.r#match('/')? {
                    while self.peek()? != '\n' && !self.is_at_end() {
                        self.advance()?;
                    }
                } else {
                    self.tokens.push(Tokens::Slash)
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

        let text = &self.source[self.start..self.current];
        let keyword_kind = KeywordKind::get(text);

        if let Some(keyword_kind) = keyword_kind {
            self.tokens.push(Tokens::Keyword(keyword_kind));
        } else {
            self.tokens.push(Tokens::Identifier(text.to_string()));
        }

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
        self.tokens.push(Tokens::Number(
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
        self.tokens.push(Tokens::String(value.to_string()));

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
        let mut lexer = FabCLexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let expected_tokens = vec![
            Tokens::LeftParen,
            Tokens::RightParen,
            Tokens::LeftBrace,
            Tokens::RightBrace,
            Tokens::Comma,
            Tokens::Dot,
            Tokens::Minus,
            Tokens::Plus,
            Tokens::Star,
            Tokens::Semicolon,
            Tokens::Bang,
            Tokens::BangEqual,
            Tokens::Equal,
            Tokens::EqualEqual,
            Tokens::Less,
            Tokens::LessEqual,
            Tokens::Greater,
            Tokens::GreaterEqual,
            Tokens::Slash,
            Tokens::EoF,
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let source = "let fn if else return goto true false none while for and or myVar";
        let mut lexer = FabCLexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let expected_tokens = vec![
            Tokens::Keyword(KeywordKind::Let),
            Tokens::Keyword(KeywordKind::Fn),
            Tokens::Keyword(KeywordKind::If),
            Tokens::Keyword(KeywordKind::Else),
            Tokens::Keyword(KeywordKind::Return),
            Tokens::Keyword(KeywordKind::Goto),
            Tokens::Keyword(KeywordKind::True),
            Tokens::Keyword(KeywordKind::False),
            Tokens::Keyword(KeywordKind::None),
            Tokens::Keyword(KeywordKind::While),
            Tokens::Keyword(KeywordKind::For),
            Tokens::Keyword(KeywordKind::And),
            Tokens::Keyword(KeywordKind::Or),
            Tokens::Identifier("myVar".to_string()),
            Tokens::EoF,
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_string_and_number_literals() {
        let source = r#""hello" 123 45.67"#;
        let mut lexer = FabCLexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let expected_tokens = vec![
            Tokens::String("hello".to_string()),
            Tokens::Number(123.0),
            Tokens::Number(45.67),
            Tokens::EoF,
        ];
        assert_eq!(*tokens, expected_tokens);
    }
}
