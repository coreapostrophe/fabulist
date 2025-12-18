use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{
    ast::{
        expr::{Expr, Primary},
        literal::Literal,
        primitive::Primitive,
    },
    error::Error,
};

pub mod ast;
pub mod error;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, Error> {
        self.expression()
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

    fn consume(&mut self, expected: Token, error: Error) -> Result<&Token, Error> {
        if self.peek() == &expected {
            Ok(self.advance())
        } else {
            Err(error)
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

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.r#match(vec![Token::BangEqual, Token::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;

        while self.r#match(vec![
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.r#match(vec![Token::Minus, Token::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.r#match(vec![Token::Slash, Token::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.r#match(vec![Token::Bang, Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }

        match self.peek() {
            Token::Keyword(keyword_kind) => match keyword_kind {
                KeywordKind::True => {
                    self.advance();
                    Ok(Expr::Primary(Primary::Literal(Literal::Boolean(true))))
                }
                KeywordKind::False => {
                    self.advance();
                    Ok(Expr::Primary(Primary::Literal(Literal::Boolean(false))))
                }
                KeywordKind::None => {
                    self.advance();
                    Ok(Expr::Primary(Primary::Literal(Literal::None)))
                }
                _ => Err(Error::UnhandledKeywordLiteral),
            },
            Token::String(string) => {
                let value = string.clone();
                self.advance();
                Ok(Expr::Primary(Primary::Literal(Literal::String(value))))
            }
            Token::Number(number) => {
                let value = *number;
                self.advance();
                Ok(Expr::Primary(Primary::Literal(Literal::Number(value))))
            }
            Token::Identifier(identifier) => {
                let name = identifier.clone();
                self.advance();
                Ok(Expr::Primary(Primary::Primitive(Primitive::Identifier(
                    name,
                ))))
            }
            Token::LeftParen => {
                let expr = self.expression()?;
                self.consume(Token::RightParen, Error::UnclosedDelimiter(")".to_string()))?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(Error::UnhandledPrimaryExpression),
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use fabc_lexer::{tokens::Token, Lexer};

    use crate::ast::{
        expr::{Expr, Primary},
        literal::Literal,
    };

    use super::Parser;

    #[test]
    fn test_simple_expression() {
        let source = "1 + 2 * 3";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().expect("Failed to parse");

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(1.0)))),
                operator: Token::Plus,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(2.0)))),
                    operator: Token::Star,
                    right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(3.0)))),
                }),
            }
        );
    }
}
