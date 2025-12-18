use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{ast::stmt::Stmt, error::Error};

pub mod ast;
pub mod error;

pub trait Parsable
where
    Self: Sized,
{
    fn parse(parser: &mut Parser) -> Result<Self, Error>;
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Stmt, Error> {
        Stmt::parse(self)
    }

    fn r#match(&mut self, expected: Vec<Token>) -> bool {
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

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn punctuated<F, T>(
        &mut self,
        start: Token,
        end: Token,
        delimiter: Token,
        parser_fn: F,
    ) -> Result<Vec<T>, Error>
    where
        F: Fn(&mut Parser<'a>) -> Result<T, Error>,
    {
        self.consume(start)?;
        let mut items = Vec::new();

        while !self.is_at_end() && self.peek() != &end {
            items.push(parser_fn(self)?);
            if !self.r#match(vec![delimiter.clone()]) {
                break;
            }
        }

        self.consume(end)?;
        Ok(items)
    }

    fn consume(&mut self, expected: Token) -> Result<&Token, Error> {
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
