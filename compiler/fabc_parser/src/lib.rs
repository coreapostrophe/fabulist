use std::slice;

use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{ast::story::Story, error::Error};

pub mod ast;
pub mod error;
mod macros;

pub trait Parsable
where
    Self: Sized,
{
    fn parse(parser: &mut Parser) -> Result<Self, Error>;
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    current: usize,
    save: Option<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            current: 0,
            save: None,
        }
    }

    pub fn parse(&mut self) -> Result<Story, Error> {
        Story::parse(self)
    }

    fn r#match(&mut self, expected: &[Token<'a>]) -> bool {
        if self.is_at_end() {
            return false;
        }
        if expected.contains(self.peek()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn previous(&self) -> &Token<'a> {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token<'a> {
        if self.is_at_end() {
            return &Token::EoF;
        }
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn enclosed<F, T>(&mut self, start: Token<'a>, end: Token<'a>, parser_fn: F) -> Result<T, Error>
    where
        F: Fn(&mut Parser<'a>) -> Result<T, Error>,
    {
        self.consume(start)?;
        let result = parser_fn(self)?;
        self.consume(end)?;
        Ok(result)
    }

    fn punctuated<F, T>(
        &mut self,
        start: Token<'a>,
        end: Token<'a>,
        delimiter: Token<'a>,
        parser_fn: F,
    ) -> Result<Vec<T>, Error>
    where
        F: Fn(&mut Parser<'a>) -> Result<T, Error>,
    {
        self.enclosed(start, end.clone(), |parser| {
            let mut items = Vec::new();

            while !parser.is_at_end() && parser.peek() != &end {
                items.push(parser_fn(parser)?);
                if !parser.r#match(slice::from_ref(&delimiter)) {
                    break;
                }
            }

            Ok(items)
        })
    }

    fn rollbacking<F, T>(&mut self, parser_fn: F) -> Option<T>
    where
        F: Fn(&mut Parser<'a>) -> Result<T, Error>,
    {
        self.save = Some(self.current);

        if let Ok(result) = parser_fn(self) {
            self.save = None;
            Some(result)
        } else {
            if let Some(saved_position) = self.save {
                self.current = saved_position;
            }
            self.save = None;
            None
        }
    }

    fn consume(&mut self, expected: Token<'a>) -> Result<&Token<'a>, Error> {
        if self.peek() == &expected {
            Ok(self.advance())
        } else {
            Err(Error::ExpectedFound {
                expected: expected.to_string(),
                found: self.peek().to_string(),
            })
        }
    }

    fn _synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let Token::Semicolon = self.previous() {
                return;
            }

            if let Token::Keyword(
                KeywordKind::Let
                | KeywordKind::Fn
                | KeywordKind::For
                | KeywordKind::If
                | KeywordKind::While
                | KeywordKind::Return,
            ) = self.peek()
            {
                return;
            }

            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use fabc_lexer::Lexer;

    use crate::Parser;

    #[test]
    fn parses_simple_story() {
        let source = fabc_reg_test::SIMPLE_STORY;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(&tokens);
        let ast = parser.parse();

        assert!(ast.is_ok());
    }

    #[test]
    fn parses_complex_story() {
        let source = fabc_reg_test::COMPLEX_STORY;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(&tokens);
        let ast = parser.parse();

        assert!(ast.is_ok());
    }
}
