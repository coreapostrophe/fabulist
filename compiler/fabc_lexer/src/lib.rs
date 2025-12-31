use crate::{
    keywords::KeywordKind,
    tokens::{ErrorKind, Token, TokenKind},
};

pub mod keywords;
pub mod tokens;

pub struct Lexer<'a> {
    tokens: Vec<Token<'a>>,
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn tokenize(input: &'a str) -> Vec<Token<'a>> {
        let mut lexer = Self {
            tokens: Vec::new(),
            source: input,
            start: 0,
            current: 0,
            line: 1,
        };

        lexer.scan_tokens();

        lexer.tokens
    }

    pub fn push_token(&mut self, kind: TokenKind<'a>) {
        let column = match kind {
            TokenKind::EoF => self.current,
            _ => self.start,
        };

        let length = match kind {
            TokenKind::EoF => 0,
            _ => self.current - self.start,
        };

        self.tokens.push(Token {
            kind,
            line: self.line,
            column,
            length,
        });
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.reset_start();
            self.scan_token();
        }
        self.push_token(TokenKind::EoF);
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // Single-character tokens.
            '(' => self.push_token(TokenKind::LeftParen),
            ')' => self.push_token(TokenKind::RightParen),
            '{' => self.push_token(TokenKind::LeftBrace),
            '}' => self.push_token(TokenKind::RightBrace),
            '[' => self.push_token(TokenKind::LeftBracket),
            ']' => self.push_token(TokenKind::RightBracket),
            ',' => self.push_token(TokenKind::Comma),
            '.' => self.push_token(TokenKind::Dot),
            '-' => self.push_token(TokenKind::Minus),
            '+' => self.push_token(TokenKind::Plus),
            '*' => self.push_token(TokenKind::Asterisk),
            ':' => self.push_token(TokenKind::Colon),
            ';' => self.push_token(TokenKind::Semicolon),
            '#' => self.push_token(TokenKind::Pound),

            // Double-character tokens.
            '!' => {
                if self.r#match('=') {
                    self.push_token(TokenKind::BangEqual)
                } else {
                    self.push_token(TokenKind::Bang)
                }
            }
            '=' => {
                if self.r#match('=') {
                    self.push_token(TokenKind::EqualEqual)
                } else if self.r#match('>') {
                    self.push_token(TokenKind::ArrowRight)
                } else {
                    self.push_token(TokenKind::Equal)
                }
            }
            '<' => {
                if self.r#match('=') {
                    self.push_token(TokenKind::LessEqual)
                } else {
                    self.push_token(TokenKind::Less)
                }
            }
            '>' => {
                if self.r#match('=') {
                    self.push_token(TokenKind::GreaterEqual)
                } else {
                    self.push_token(TokenKind::Greater)
                }
            }

            // Comments and whitespace.
            '/' => {
                if self.r#match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.push_token(TokenKind::Slash)
                }
            }
            ' ' | '\r' | '\t' | '\n' => {
                while self.is_white_space() {
                    if c == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }
            }

            // Literals.
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => self.push_token(TokenKind::Error(ErrorKind::UnrecognizedCharacter)),
        }
    }

    fn is_white_space(&self) -> bool {
        matches!(self.peek(), ' ' | '\r' | '\t' | '\n')
    }

    fn reset_start(&mut self) {
        self.start = self.current;
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let keyword_kind = KeywordKind::get(text);

        if let Some(keyword_kind) = keyword_kind {
            self.push_token(TokenKind::Keyword(keyword_kind));
        } else {
            self.push_token(TokenKind::Identifier(text));
        }
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number_str = &self.source[self.start..self.current];

        if let Ok(number) = number_str.parse::<f64>() {
            self.push_token(TokenKind::Number(number));
        } else {
            self.push_token(TokenKind::Error(ErrorKind::InvalidNumber));
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.push_token(TokenKind::Error(ErrorKind::UnterminatedString));
        } else {
            self.advance();
            let value = &self.source[self.start + 1..self.current - 1];
            self.push_token(TokenKind::String(value));
        }
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.peek() != expected {
            return false;
        }
        self.advance();
        true
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

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let ch = self.peek();
        self.current += 1;
        ch
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
        let tokens = Lexer::tokenize(source);
        let expected_tokens = vec![
            Token {
                kind: TokenKind::LeftParen,
                line: 1,
                column: 0,
                length: 1,
            },
            Token {
                kind: TokenKind::RightParen,
                line: 1,
                column: 2,
                length: 1,
            },
            Token {
                kind: TokenKind::LeftBrace,
                line: 1,
                column: 4,
                length: 1,
            },
            Token {
                kind: TokenKind::RightBrace,
                line: 1,
                column: 6,
                length: 1,
            },
            Token {
                kind: TokenKind::LeftBracket,
                line: 1,
                column: 8,
                length: 1,
            },
            Token {
                kind: TokenKind::RightBracket,
                line: 1,
                column: 10,
                length: 1,
            },
            Token {
                kind: TokenKind::Comma,
                line: 1,
                column: 12,
                length: 1,
            },
            Token {
                kind: TokenKind::Dot,
                line: 1,
                column: 14,
                length: 1,
            },
            Token {
                kind: TokenKind::Minus,
                line: 1,
                column: 16,
                length: 1,
            },
            Token {
                kind: TokenKind::Plus,
                line: 1,
                column: 18,
                length: 1,
            },
            Token {
                kind: TokenKind::Asterisk,
                line: 1,
                column: 20,
                length: 1,
            },
            Token {
                kind: TokenKind::Colon,
                line: 1,
                column: 22,
                length: 1,
            },
            Token {
                kind: TokenKind::Semicolon,
                line: 1,
                column: 24,
                length: 1,
            },
            Token {
                kind: TokenKind::Bang,
                line: 1,
                column: 26,
                length: 1,
            },
            Token {
                kind: TokenKind::BangEqual,
                line: 1,
                column: 28,
                length: 2,
            },
            Token {
                kind: TokenKind::Equal,
                line: 1,
                column: 31,
                length: 1,
            },
            Token {
                kind: TokenKind::EqualEqual,
                line: 1,
                column: 33,
                length: 2,
            },
            Token {
                kind: TokenKind::Less,
                line: 1,
                column: 36,
                length: 1,
            },
            Token {
                kind: TokenKind::LessEqual,
                line: 1,
                column: 38,
                length: 2,
            },
            Token {
                kind: TokenKind::Greater,
                line: 1,
                column: 41,
                length: 1,
            },
            Token {
                kind: TokenKind::GreaterEqual,
                line: 1,
                column: 43,
                length: 2,
            },
            Token {
                kind: TokenKind::Slash,
                line: 1,
                column: 46,
                length: 1,
            },
            Token {
                kind: TokenKind::ArrowRight,
                line: 1,
                column: 48,
                length: 2,
            },
            Token {
                kind: TokenKind::EoF,
                line: 1,
                column: 50,
                length: 0,
            },
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let source = "let fn if else return goto true false none while for and or context myVar";
        let tokens = Lexer::tokenize(source);
        let expected_tokens = vec![
            Token {
                kind: TokenKind::Keyword(KeywordKind::Let),
                line: 1,
                column: 0,
                length: 3,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Fn),
                line: 1,
                column: 4,
                length: 2,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::If),
                line: 1,
                column: 7,
                length: 2,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Else),
                line: 1,
                column: 10,
                length: 4,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Return),
                line: 1,
                column: 15,
                length: 6,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Goto),
                line: 1,
                column: 22,
                length: 4,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::True),
                line: 1,
                column: 27,
                length: 4,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::False),
                line: 1,
                column: 32,
                length: 5,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::None),
                line: 1,
                column: 38,
                length: 4,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::While),
                line: 1,
                column: 43,
                length: 5,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::For),
                line: 1,
                column: 49,
                length: 3,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::And),
                line: 1,
                column: 53,
                length: 3,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Or),
                line: 1,
                column: 57,
                length: 2,
            },
            Token {
                kind: TokenKind::Keyword(KeywordKind::Context),
                line: 1,
                column: 60,
                length: 7,
            },
            Token {
                kind: TokenKind::Identifier("myVar"),
                line: 1,
                column: 68,
                length: 5,
            },
            Token {
                kind: TokenKind::EoF,
                line: 1,
                column: 73,
                length: 0,
            },
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_string_and_number_literals() {
        let source = r#""hello" 123 45.67"#;
        let tokens = Lexer::tokenize(source);
        let expected_tokens = vec![
            Token {
                kind: TokenKind::String("hello"),
                line: 1,
                column: 0,
                length: 7,
            },
            Token {
                kind: TokenKind::Number(123.0),
                line: 1,
                column: 8,
                length: 3,
            },
            Token {
                kind: TokenKind::Number(45.67),
                line: 1,
                column: 12,
                length: 5,
            },
            Token {
                kind: TokenKind::EoF,
                line: 1,
                column: 17,
                length: 0,
            },
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn test_error_token() {
        let source = "@ \"fasfa";
        let tokens = Lexer::tokenize(source);
        let expected_tokens = vec![
            Token {
                kind: TokenKind::Error(ErrorKind::UnrecognizedCharacter),
                line: 1,
                column: 0,
                length: 1,
            },
            Token {
                kind: TokenKind::Error(ErrorKind::UnterminatedString),
                line: 1,
                column: 2,
                length: 6,
            },
            Token {
                kind: TokenKind::EoF,
                line: 1,
                column: 8,
                length: 0,
            },
        ];
        assert_eq!(*tokens, expected_tokens);
    }

    #[test]
    fn tokenizes_simple_story() {
        let tokens = Lexer::tokenize(fabc_reg_test::SIMPLE_STORY);
        assert!(tokens
            .iter()
            .all(|token| !matches!(token.kind, TokenKind::Error(_))));
        assert!(!tokens.is_empty());
    }

    #[test]
    fn tokenizes_complex_story() {
        let tokens = Lexer::tokenize(fabc_reg_test::COMPLEX_STORY);
        assert!(tokens
            .iter()
            .all(|token| !matches!(token.kind, TokenKind::Error(_))));
        assert!(!tokens.is_empty());
    }
}
