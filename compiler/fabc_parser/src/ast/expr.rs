use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{
    ast::expr::{literal::Literal, primitive::Primitive},
    error::Error,
    Parsable, Parser,
};

pub mod literal;
pub mod primitive;

#[derive(Debug, PartialEq)]
pub enum Primary {
    Literal(Literal),
    Primitive(Primitive),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Primary(Primary),
    Grouping(Box<Expr>),
}

impl Expr {
    pub fn equality(parser: &mut Parser) -> Result<Expr, Error> {
        let mut expr = Self::comparison(parser)?;

        while parser.r#match(vec![Token::BangEqual, Token::EqualEqual]) {
            let operator = parser.previous().clone();
            let right = Self::comparison(parser)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(parser: &mut Parser) -> Result<Expr, Error> {
        let mut expr = Self::term(parser)?;

        while parser.r#match(vec![
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let operator = parser.previous().clone();
            let right = Self::term(parser)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(parser: &mut Parser) -> Result<Expr, Error> {
        let mut expr = Self::factor(parser)?;

        while parser.r#match(vec![Token::Minus, Token::Plus]) {
            let operator = parser.previous().clone();
            let right = Self::factor(parser)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(parser: &mut Parser) -> Result<Expr, Error> {
        let mut expr = Self::unary(parser)?;

        while parser.r#match(vec![Token::Slash, Token::Asterisk]) {
            let operator = parser.previous().clone();
            let right = Self::unary(parser)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(parser: &mut Parser) -> Result<Expr, Error> {
        if parser.r#match(vec![Token::Bang, Token::Minus]) {
            let operator = parser.previous().clone();
            let right = Self::unary(parser)?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        Self::primary(parser)
    }

    fn primary(parser: &mut Parser) -> Result<Expr, Error> {
        if parser.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }

        match parser.peek() {
            Token::String(_)
            | Token::Number(_)
            | Token::Keyword(KeywordKind::True | KeywordKind::False | KeywordKind::None) => {
                let literal = Literal::parse(parser)?;
                Ok(Expr::Primary(Primary::Literal(literal)))
            }
            Token::Identifier(_) | Token::Path(_) => {
                let primitive = Primitive::parse(parser)?;
                Ok(Expr::Primary(Primary::Primitive(primitive)))
            }
            Token::LeftParen => {
                parser.consume(Token::LeftParen)?;
                let expr = Expr::parse(parser)?;
                parser.consume(Token::RightParen)?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(Error::UnhandledPrimaryExpression),
        }
    }
}

impl Parsable for Expr {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Self::equality(parser)
    }
}

#[cfg(test)]
mod expr_tests {
    use fabc_lexer::{tokens::Token, Lexer};

    use crate::{
        ast::expr::{literal::Literal, Expr, Primary},
        Parsable, Parser,
    };

    #[test]
    fn parses_arithmetic_binary_expr() {
        let source = "1 + 2 * 3 / 4";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = Expr::parse(&mut parser).expect("Failed to parse");

        let expected = Expr::Binary {
            left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(1.0)))),
            operator: Token::Plus,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(2.0)))),
                    operator: Token::Asterisk,
                    right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(3.0)))),
                }),
                operator: Token::Slash,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(4.0)))),
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_equality_expr() {
        let source = "10 == 20 != 30";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = Expr::parse(&mut parser).expect("Failed to parse");

        let expected = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(10.0)))),
                operator: Token::EqualEqual,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(20.0)))),
            }),
            operator: Token::BangEqual,
            right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(30.0)))),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_comparison_expr() {
        let source = "5 > 3 < 9 >= 2 <= 10";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = Expr::parse(&mut parser).expect("Failed to parse");

        let expected = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(5.0)))),
                        operator: Token::Greater,
                        right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(3.0)))),
                    }),
                    operator: Token::Less,
                    right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(9.0)))),
                }),
                operator: Token::GreaterEqual,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(2.0)))),
            }),
            operator: Token::LessEqual,
            right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(10.0)))),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_unary_expr() {
        let source = "-!42";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = Expr::parse(&mut parser).expect("Failed to parse");

        let expected = Expr::Unary {
            operator: Token::Minus,
            right: Box::new(Expr::Unary {
                operator: Token::Bang,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(42.0)))),
            }),
        };

        assert_eq!(expr, expected);
    }
}
