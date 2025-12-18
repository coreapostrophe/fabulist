use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{
    ast::expr::{literal::Literal, primitive::Primitive},
    error::Error,
    Parsable, Parser,
};

pub mod literal;
pub mod primitive;

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    EqualEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Add,
    Subtraction,
    Multiply,
    Divide,
    And,
    Or,
}

impl TryFrom<&Token> for BinaryOperator {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::EqualEqual => Ok(BinaryOperator::EqualEqual),
            Token::BangEqual => Ok(BinaryOperator::NotEqual),
            Token::Greater => Ok(BinaryOperator::Greater),
            Token::GreaterEqual => Ok(BinaryOperator::GreaterEqual),
            Token::Less => Ok(BinaryOperator::Less),
            Token::LessEqual => Ok(BinaryOperator::LessEqual),
            Token::Plus => Ok(BinaryOperator::Add),
            Token::Minus => Ok(BinaryOperator::Subtraction),
            Token::Asterisk => Ok(BinaryOperator::Multiply),
            Token::Slash => Ok(BinaryOperator::Divide),
            _ => Err(Error::InvalidBinaryOperator),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
}

impl TryFrom<&Token> for UnaryOperator {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Bang => Ok(UnaryOperator::Not),
            Token::Minus => Ok(UnaryOperator::Negate),
            _ => Err(Error::InvalidUnaryOperator),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl TryFrom<&Token> for LogicalOperator {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Keyword(KeywordKind::And) => Ok(LogicalOperator::And),
            Token::Keyword(KeywordKind::Or) => Ok(LogicalOperator::Or),
            _ => Err(Error::InvalidLogicalOperator),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Primary {
    Literal(Literal),
    Primitive(Primitive),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expr>,
    },
    Assignment {
        name: Box<Expr>,
        value: Box<Expr>,
    },
    Primary(Primary),
    Grouping(Box<Expr>),
}

impl Expr {
    pub fn assignment(parser: &mut Parser) -> Result<Expr, Error> {
        let mut expr = Self::logical(parser)?;

        if parser.r#match(vec![Token::Equal]) {
            let value = Self::assignment(parser)?;
            expr = Expr::Assignment {
                name: Box::new(expr),
                value: Box::new(value),
            }
        }

        Ok(expr)
    }

    fn logical(parser: &mut Parser) -> Result<Expr, Error> {
        let mut expr = Self::equality(parser)?;

        while parser.r#match(vec![
            Token::Keyword(KeywordKind::And),
            Token::Keyword(KeywordKind::Or),
        ]) {
            let operator = LogicalOperator::try_from(parser.previous())?;
            let right = Self::equality(parser)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: match operator {
                    LogicalOperator::And => BinaryOperator::And,
                    LogicalOperator::Or => BinaryOperator::Or,
                },
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(parser: &mut Parser) -> Result<Expr, Error> {
        let mut expr = Self::comparison(parser)?;

        while parser.r#match(vec![Token::BangEqual, Token::EqualEqual]) {
            let operator = BinaryOperator::try_from(parser.previous())?;
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
            let operator = BinaryOperator::try_from(parser.previous())?;
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
            let operator = BinaryOperator::try_from(parser.previous())?;
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
            let operator = BinaryOperator::try_from(parser.previous())?;
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
            let operator = UnaryOperator::try_from(parser.previous())?;
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
        Self::assignment(parser)
    }
}

#[cfg(test)]
mod expr_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::expr::{
            literal::Literal, primitive::Primitive, BinaryOperator, Expr, Primary, UnaryOperator,
        },
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
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(2.0)))),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(3.0)))),
                }),
                operator: BinaryOperator::Divide,
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
                operator: BinaryOperator::EqualEqual,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(20.0)))),
            }),
            operator: BinaryOperator::NotEqual,
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
                        operator: BinaryOperator::Greater,
                        right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(3.0)))),
                    }),
                    operator: BinaryOperator::Less,
                    right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(9.0)))),
                }),
                operator: BinaryOperator::GreaterEqual,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(2.0)))),
            }),
            operator: BinaryOperator::LessEqual,
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
            operator: UnaryOperator::Negate,
            right: Box::new(Expr::Unary {
                operator: UnaryOperator::Not,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(42.0)))),
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_grouping_expr() {
        let source = "(1 + 2) * 3";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = Expr::parse(&mut parser).expect("Failed to parse");

        let expected = Expr::Binary {
            left: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(1.0)))),
                operator: BinaryOperator::Add,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(2.0)))),
            }))),
            operator: BinaryOperator::Multiply,
            right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(3.0)))),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_logical_expr() {
        let source = "true and false or true";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = Expr::parse(&mut parser).expect("Failed to parse");

        let expected = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Primary(Primary::Literal(Literal::Boolean(true)))),
                operator: BinaryOperator::And,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Boolean(false)))),
            }),
            operator: BinaryOperator::Or,
            right: Box::new(Expr::Primary(Primary::Literal(Literal::Boolean(true)))),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_assignment_expr() {
        let source = "x = 10 + 20";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = Expr::parse(&mut parser).expect("Failed to parse");

        let expected = Expr::Assignment {
            name: Box::new(Expr::Primary(Primary::Primitive(Primitive::Identifier(
                "x".to_string(),
            )))),
            value: Box::new(Expr::Binary {
                left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(10.0)))),
                operator: BinaryOperator::Add,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(20.0)))),
            }),
        };

        assert_eq!(expr, expected);
    }
}
