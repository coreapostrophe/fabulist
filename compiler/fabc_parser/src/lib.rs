use std::slice;

use fabc_error::{kind::CompileErrorKind, Error, LineCol};
use fabc_lexer::{
    tokens::{Token, TokenKind},
    Lexer,
};

use crate::ast::init::Init;

pub mod ast;
mod macros;

pub trait Parsable
where
    Self: Sized,
{
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error>;
}

pub struct Save {
    current: usize,
    id_counter: usize,
}

#[derive(Debug)]
pub struct ParserResult<T> {
    pub result: T,
    pub errors: Vec<Error>,
}

pub struct Parser<'src, 'tok> {
    tokens: &'tok [Token<'src>],
    current: usize,
    save: Option<Save>,
    id_counter: usize,
    #[allow(unused)]
    errors: Vec<Error>,
}

impl<'src, 'tok> Parser<'src, 'tok> {
    pub fn parse_str(source: &str) -> ParserResult<Vec<Init>> {
        let tokens = Lexer::tokenize(source);
        Parser::parse(&tokens)
    }

    pub fn parse(tokens: &[Token<'src>]) -> ParserResult<Vec<Init>> {
        let mut parser = Parser {
            tokens,
            current: 0,
            save: None,
            id_counter: 0,
            errors: Vec::new(),
        };

        let inits = parser.invariant_parse::<Init>(Init::SYNC_DELIMITERS, &[], false);

        ParserResult {
            result: inits,
            errors: parser.errors,
        }
    }

    pub fn parse_ast_str<T>(source: &str) -> Result<T, Error>
    where
        T: Parsable,
    {
        let tokens = Lexer::tokenize(source);
        Parser::parse_ast::<T>(&tokens)
    }

    pub fn parse_ast<T>(tokens: &[Token<'src>]) -> Result<T, Error>
    where
        T: Parsable,
    {
        let mut parser = Parser {
            tokens,
            current: 0,
            save: None,
            id_counter: 0,
            errors: Vec::new(),
        };

        T::parse(&mut parser)
    }

    pub(crate) fn start_span(&self) -> LineCol {
        LineCol::from_token(self.peek_token())
    }

    pub(crate) fn end_span(&self) -> LineCol {
        LineCol::from_token_end(self.previous_token())
    }

    pub(crate) fn push_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub(crate) fn assign_id(&mut self) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    pub(crate) fn r#match(&mut self, expected: &[TokenKind<'src>]) -> bool {
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

    pub(crate) fn previous(&self) -> &TokenKind<'src> {
        &self.tokens[self.current - 1].kind
    }

    pub(crate) fn previous_token(&self) -> &Token<'src> {
        &self.tokens[self.current - 1]
    }

    pub(crate) fn peek(&self) -> &TokenKind<'src> {
        if self.is_at_end() {
            return &TokenKind::EoF;
        }
        &self.tokens[self.current].kind
    }

    pub(crate) fn peek_token(&self) -> &Token<'src> {
        &self.tokens[self.current]
    }

    pub(crate) fn advance(&mut self) -> &TokenKind<'src> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub(crate) fn enclosed<F, T>(
        &mut self,
        start: TokenKind<'src>,
        end: TokenKind<'src>,
        mut parser_fn: F,
    ) -> Result<T, Error>
    where
        F: FnMut(&mut Parser<'src, 'tok>) -> Result<T, Error>,
    {
        let delimiter_start_index = self.current;
        self.consume(start)?;
        let result = parser_fn(self)?;
        if self.consume(end).is_err() {
            return Err(Error::new(
                CompileErrorKind::UnclosedDelimiter,
                &self.tokens[delimiter_start_index],
            ));
        }
        Ok(result)
    }

    pub(crate) fn punctuated<F, T>(
        &mut self,
        start: TokenKind<'src>,
        end: TokenKind<'src>,
        delimiter: TokenKind<'src>,
        mut parser_fn: F,
    ) -> Result<Vec<T>, Error>
    where
        F: FnMut(&mut Parser<'src, 'tok>) -> Result<T, Error>,
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

    pub(crate) fn rollbacking<F, T>(&mut self, mut parser_fn: F) -> Option<T>
    where
        F: FnMut(&mut Parser<'src, 'tok>) -> Result<T, Error>,
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

    pub(crate) fn consume(&mut self, expected: TokenKind<'src>) -> Result<&TokenKind<'src>, Error> {
        if self.peek() == &expected {
            Ok(self.advance())
        } else {
            Err(Error::new(
                CompileErrorKind::ExpectedSymbol {
                    expected: expected.to_string(),
                    found: self.peek().to_string(),
                },
                self.peek_token(),
            ))
        }
    }

    pub(crate) fn invariant_parse<T>(
        &mut self,
        sync_delimiters: &[TokenKind<'src>],
        sync_end_tokens: &[TokenKind<'src>],
        consume_delimiter: bool,
    ) -> Vec<T>
    where
        T: Parsable,
    {
        let mut ast_list = Vec::new();

        while if !sync_end_tokens.is_empty() {
            !self.is_terminated() && !sync_end_tokens.contains(self.peek())
        } else {
            !self.is_terminated()
        } {
            match T::parse(self) {
                Ok(parsed_ast) => ast_list.push(parsed_ast),
                Err(error) => {
                    self.push_error(error);
                    while !self.is_terminated() && !sync_delimiters.contains(self.peek()) {
                        self.advance();
                    }
                    if !self.is_terminated() && consume_delimiter {
                        self.advance();
                    }
                }
            }
        }

        ast_list
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub(crate) fn is_terminated(&self) -> bool {
        self.is_at_end() || self.peek() == &TokenKind::EoF
    }
}

#[cfg(test)]
mod tests {
    use fabc_lexer::Lexer;

    use crate::Parser;

    #[test]
    fn parses_simple_story() {
        let source = fabc_reg_test::SIMPLE_STORY;
        let tokens = Lexer::tokenize(source);
        let story = Parser::parse(&tokens);

        assert!(story.errors.is_empty());
    }

    #[test]
    fn parses_complex_story() {
        let source = fabc_reg_test::COMPLEX_STORY;
        let tokens = Lexer::tokenize(source);
        let story = Parser::parse(&tokens);

        println!("Errors: {:?}", story.errors);

        assert!(story.errors.is_empty());
    }
}
