use crate::{
    error::Error,
    keywords::KeywordKind,
    tokens::{Token, TokenKind},
};

pub mod error;
pub mod keywords;
pub mod tokens;

pub struct Lexer<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn tokenize(input: &'a str) -> Result<Vec<Token<'a>>, Error> {
        let mut lexer = Self {
            source: input,
            start: 0,
            current: 0,
            line: 1,
        };

        lexer.scan_tokens()
    }

    pub fn make_token(&self, kind: TokenKind<'a>) -> Token<'a> {
        Token {
            kind,
            line: self.line,
            column: self.start,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token<'a>>, Error> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.reset_start();
            let token = self.scan_token()?;
            tokens.push(token);
        }
        tokens.push(self.make_token(TokenKind::EoF));

        Ok(tokens)
    }

    pub fn scan_token(&mut self) -> Result<Token<'a>, Error> {
        if self.is_at_end() {
            return Ok(self.make_token(TokenKind::EoF));
        }

        let c = self.advance()?;

        match c {
            // Single-character tokens.
            '(' => Ok(self.make_token(TokenKind::LeftParen)),
            ')' => Ok(self.make_token(TokenKind::RightParen)),
            '{' => Ok(self.make_token(TokenKind::LeftBrace)),
            '}' => Ok(self.make_token(TokenKind::RightBrace)),
            '[' => Ok(self.make_token(TokenKind::LeftBracket)),
            ']' => Ok(self.make_token(TokenKind::RightBracket)),
            ',' => Ok(self.make_token(TokenKind::Comma)),
            '.' => Ok(self.make_token(TokenKind::Dot)),
            '-' => Ok(self.make_token(TokenKind::Minus)),
            '+' => Ok(self.make_token(TokenKind::Plus)),
            '*' => Ok(self.make_token(TokenKind::Asterisk)),
            ':' => Ok(self.make_token(TokenKind::Colon)),
            ';' => Ok(self.make_token(TokenKind::Semicolon)),
            '#' => Ok(self.make_token(TokenKind::Pound)),

            // Double-character tokens.
            '!' => {
                if self.r#match('=')? {
                    Ok(self.make_token(TokenKind::BangEqual))
                } else {
                    Ok(self.make_token(TokenKind::Bang))
                }
            }
            '=' => {
                if self.r#match('=')? {
                    Ok(self.make_token(TokenKind::EqualEqual))
                } else if self.r#match('>')? {
                    Ok(self.make_token(TokenKind::ArrowRight))
                } else {
                    Ok(self.make_token(TokenKind::Equal))
                }
            }
            '<' => {
                if self.r#match('=')? {
                    Ok(self.make_token(TokenKind::LessEqual))
                } else {
                    Ok(self.make_token(TokenKind::Less))
                }
            }
            '>' => {
                if self.r#match('=')? {
                    Ok(self.make_token(TokenKind::GreaterEqual))
                } else {
                    Ok(self.make_token(TokenKind::Greater))
                }
            }

            // Comments and whitespace.
            '/' => {
                if self.r#match('/')? {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance()?;
                    }
                    self.scan_token()
                } else {
                    Ok(self.make_token(TokenKind::Slash))
                }
            }
            ' ' | '\r' | '\t' | '\n' => {
                while self.is_white_space() {
                    if c == '\n' {
                        self.line += 1;
                    }
                    self.advance()?;
                }
                self.reset_start();
                self.scan_token()
            }

            // Literals.
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => Err(Error::UnexpectedCharacter(c)),
        }
    }

    fn is_white_space(&self) -> bool {
        matches!(self.peek(), ' ' | '\r' | '\t' | '\n')
    }

    fn reset_start(&mut self) {
        self.start = self.current;
    }

    fn identifier(&mut self) -> Result<Token<'a>, Error> {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance()?;
        }

        let text = &self.source[self.start..self.current];
        let keyword_kind = KeywordKind::get(text);

        if let Some(keyword_kind) = keyword_kind {
            Ok(self.make_token(TokenKind::Keyword(keyword_kind)))
        } else {
            Ok(self.make_token(TokenKind::Identifier(text)))
        }
    }

    fn number(&mut self) -> Result<Token<'a>, Error> {
        while self.peek().is_ascii_digit() {
            self.advance()?;
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance()?;

            while self.peek().is_ascii_digit() {
                self.advance()?;
            }
        }

        let number_str = &self.source[self.start..self.current];

        Ok(self.make_token(TokenKind::Number(
            number_str.parse().map_err(|_| Error::UnableToParseNumber)?,
        )))
    }

    fn string(&mut self) -> Result<Token<'a>, Error> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance()?;
        }

        if self.is_at_end() {
            return Err(Error::UnterminatedString);
        }

        self.advance()?;

        let value = &self.source[self.start + 1..self.current - 1];
        Ok(self.make_token(TokenKind::String(value)))
    }

    fn r#match(&mut self, expected: char) -> Result<bool, Error> {
        if self.peek() != expected {
            return Ok(false);
        }
        self.advance()?;
        Ok(true)
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.current] as char
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.current + 1] as char
        }
    }

    fn advance(&mut self) -> Result<char, Error> {
        if self.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }
        let ch = self.peek();
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
        let source = "( ) { } [ ] , . - + * : ; ! != = == < <= > >= / =>";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");
        let expected_tokens = vec![
            Token {
                kind: TokenKind::LeftParen,
                line: 1,
                column: 0,
            },
            Token {
                kind: TokenKind::RightParen,
                line: 1,
                column: 2,
            },
            Token {
                kind: TokenKind::LeftBrace,
                line: 1,
                column: 4,
            },
            Token {
                kind: TokenKind::RightBrace,
                line: 1,
                column: 6,
            },
            Token {
                kind: TokenKind::LeftBracket,
                line: 1,
                column: 8,
            },
            Token {
                kind: TokenKind::RightBracket,
                line: 1,
                column: 10,
            },
            Token {
                kind: TokenKind::Comma,
                line: 1,
                column: 12,
            },
            Token {
                kind: TokenKind::Dot,
                line: 1,
                column: 14,
            },
            Token {
                kind: TokenKind::Minus,
                line: 1,
                column: 16,
            },
            Token {
                kind: TokenKind::Plus,
                line: 1,
                column: 18,
            },
            Token {
                kind: TokenKind::Asterisk,
                line: 1,
                column: 20,
            },
            Token {
                kind: TokenKind::Colon,
                line: 1,
                column: 22,
            },
            Token {
                kind: TokenKind::Semicolon,
                line: 1,
                column: 24,
            },
            Token {
                kind: TokenKind::Bang,
                line: 1,
                column: 26,
            },
            Token {
                kind: TokenKind::BangEqual,
                line: 1,
                column: 28,
            },
            Token {
                kind: TokenKind::Equal,
                line: 1,
                column: 31,
            },
            Token {
                kind: TokenKind::EqualEqual,
                line: 1,
                column: 33,
            },
            Token {
                kind: TokenKind::Less,
                line: 1,
                column: 36,
            },
            Token {
                kind: TokenKind::LessEqual,
                line: 1,
                column: 38,
            },
            Token {
                kind: TokenKind::Greater,
                line: 1,
                column: 41,
            },
            Token {
                kind: TokenKind::GreaterEqual,
                line: 1,
                column: 43,
            },
            Token {
                kind: TokenKind::Slash,
                line: 1,
                column: 46,
            },
            Token {
                kind: TokenKind::ArrowRight,
                line: 1,
                column: 48,
            },
            Token {
                kind: TokenKind::EoF,
                line: 1,
                column: 48,
            },
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let source = "let fn if else return goto true false none while for and or context myVar";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");
        let expected_tokens = vec![
            Token {
                kind: TokenKind::Keyword(KeywordKind::Let),
                line: 1,
                column: 0,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Fn),
                line: 1,
                column: 4,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::If),
                line: 1,
                column: 7,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Else),
                line: 1,
                column: 10,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Return),
                line: 1,
                column: 15,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Goto),
                line: 1,
                column: 22,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::True),
                line: 1,
                column: 27,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::False),
                line: 1,
                column: 32,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::None),
                line: 1,
                column: 38,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::While),
                line: 1,
                column: 43,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::For),
                line: 1,
                column: 49,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::And),
                line: 1,
                column: 53,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Or),
                line: 1,
                column: 57,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Context),
                line: 1,
                column: 60,
            },
            Token {
                kind: TokenKind::Identifier("myVar"),
                line: 1,
                column: 68,
            },
            Token {
                kind: TokenKind::EoF,
                line: 1,
                column: 68,
            },
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_string_and_number_literals() {
        let source = r#""hello" 123 45.67"#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");
        let expected_tokens = vec![
            Token {
                kind: TokenKind::String("hello"),
                line: 1,
                column: 0,
            },
            Token {
                kind: TokenKind::Number(123.0),
                line: 1,
                column: 8,
            },
            Token {
                kind: TokenKind::Number(45.67),
                line: 1,
                column: 12,
            },
            Token {
                kind: TokenKind::EoF,
                line: 1,
                column: 12,
            },
        ];
        assert_eq!(*tokens, expected_tokens);
    }
}
