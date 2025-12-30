use std::slice;

use fabc_lexer::{
    keywords::KeywordKind,
    tokens::{Token, TokenKind},
    Lexer,
};

use crate::error::Error;

pub mod ast;
pub mod error;
mod macros;

pub trait Parsable
where
    Self: Sized,
{
    fn parse<'src, 'tok>(parser: &mut Parser<'src, 'tok>) -> Result<Self, Error>;
}

pub struct Save {
    current: usize,
    id_counter: usize,
}

pub struct Parser<'src, 'tok> {
    tokens: &'tok [Token<'src>],
    current: usize,
    save: Option<Save>,
    id_counter: usize,
}

impl<'src, 'tok> Parser<'src, 'tok> {
    pub fn parse_str<T>(source: &str) -> Result<T, Error>
    where
        T: Parsable,
    {
        let tokens = Lexer::tokenize(source)?;

        let mut parser = Parser {
            tokens: &tokens,
            current: 0,
            save: None,
            id_counter: 0,
        };

        T::parse(&mut parser)
    }

    pub fn parse<T>(tokens: &'tok [Token<'src>]) -> Result<T, Error>
    where
        T: Parsable,
    {
        let mut parser = Self {
            tokens,
            current: 0,
            save: None,
            id_counter: 0,
        };

        T::parse(&mut parser)
    }

    fn assign_id(&mut self) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    fn r#match(&mut self, expected: &[TokenKind<'src>]) -> bool {
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

    fn previous(&self) -> &TokenKind<'src> {
        &self.tokens[self.current - 1].kind
    }

    fn peek(&self) -> &TokenKind<'src> {
        if self.is_at_end() {
            return &TokenKind::EoF;
        }
        &self.tokens[self.current].kind
    }

    fn advance(&mut self) -> &TokenKind<'src> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn prefixed<F, T>(&mut self, prefix: TokenKind<'src>, parser_fn: F) -> Result<T, Error>
    where
        F: Fn(&mut Parser<'src, 'tok>) -> Result<T, Error>,
    {
        self.consume(prefix)?;
        parser_fn(self)
    }

    fn enclosed<F, T>(
        &mut self,
        start: TokenKind<'src>,
        end: TokenKind<'src>,
        parser_fn: F,
    ) -> Result<T, Error>
    where
        F: Fn(&mut Parser<'src, 'tok>) -> Result<T, Error>,
    {
        self.consume(start)?;
        let result = parser_fn(self)?;
        self.consume(end)?;
        Ok(result)
    }

    fn punctuated<F, T>(
        &mut self,
        start: TokenKind<'src>,
        end: TokenKind<'src>,
        delimiter: TokenKind<'src>,
        parser_fn: F,
    ) -> Result<Vec<T>, Error>
    where
        F: Fn(&mut Parser<'src, 'tok>) -> Result<T, Error>,
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
        F: Fn(&mut Parser<'src, 'tok>) -> Result<T, Error>,
    {
        self.save = Some(Save {
            current: self.current,
            id_counter: self.id_counter,
        });

        if let Ok(result) = parser_fn(self) {
            self.save = None;
            Some(result)
        } else {
            if let Some(saved_position) = &self.save {
                self.current = saved_position.current;
                self.id_counter = saved_position.id_counter;
            }
            self.save = None;
            None
        }
    }

    fn consume(&mut self, expected: TokenKind<'src>) -> Result<&TokenKind<'src>, Error> {
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
            if let TokenKind::Semicolon = self.previous() {
                return;
            }

            if let TokenKind::Keyword(
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

    use crate::{ast::story::Story, Parser};

    #[test]
    fn parses_simple_story() {
        let source = fabc_reg_test::SIMPLE_STORY;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let story = Parser::parse::<Story>(&tokens);

        assert!(story.is_ok());
    }

    #[test]
    fn parses_complex_story() {
        let source = fabc_reg_test::COMPLEX_STORY;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let story = Parser::parse::<Story>(&tokens);

        assert!(story.is_ok());
    }
}
