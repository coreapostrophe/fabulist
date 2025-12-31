use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

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

impl TryFrom<&TokenKind<'_>> for BinaryOperator {
    type Error = Error;

    fn try_from(token: &TokenKind) -> Result<Self, Self::Error> {
        match token {
            TokenKind::EqualEqual => Ok(BinaryOperator::EqualEqual),
            TokenKind::BangEqual => Ok(BinaryOperator::NotEqual),
            TokenKind::Greater => Ok(BinaryOperator::Greater),
            TokenKind::GreaterEqual => Ok(BinaryOperator::GreaterEqual),
            TokenKind::Less => Ok(BinaryOperator::Less),
            TokenKind::LessEqual => Ok(BinaryOperator::LessEqual),
            TokenKind::Plus => Ok(BinaryOperator::Add),
            TokenKind::Minus => Ok(BinaryOperator::Subtraction),
            TokenKind::Asterisk => Ok(BinaryOperator::Multiply),
            TokenKind::Slash => Ok(BinaryOperator::Divide),
            _ => Err(Error::InvalidBinaryOperator),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
}

impl TryFrom<&TokenKind<'_>> for UnaryOperator {
    type Error = Error;

    fn try_from(token: &TokenKind) -> Result<Self, Self::Error> {
        match token {
            TokenKind::Bang => Ok(UnaryOperator::Not),
            TokenKind::Minus => Ok(UnaryOperator::Negate),
            _ => Err(Error::InvalidUnaryOperator),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl TryFrom<&TokenKind<'_>> for LogicalOperator {
    type Error = Error;

    fn try_from(token: &TokenKind) -> Result<Self, Self::Error> {
        match token {
            TokenKind::Keyword(KeywordKind::And) => Ok(LogicalOperator::And),
            TokenKind::Keyword(KeywordKind::Or) => Ok(LogicalOperator::Or),
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
        id: usize,
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Unary {
        id: usize,
        operator: UnaryOperator,
        right: Box<Expr>,
    },
    Assignment {
        id: usize,
        name: Box<Expr>,
        value: Box<Expr>,
    },
    MemberAccess {
        id: usize,
        left: Box<Expr>,
        members: Vec<Expr>,
    },
    Call {
        id: usize,
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Primary {
        id: usize,
        value: Primary,
    },
    Grouping {
        id: usize,
        expression: Box<Expr>,
    },
}

impl Expr {
    pub fn assignment(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let mut expr = Self::logical(parser)?;

        if parser.r#match(&[TokenKind::Equal]) {
            let value = Self::assignment(parser)?;
            expr = Expr::Assignment {
                id: parser.assign_id(),
                name: Box::new(expr),
                value: Box::new(value),
            }
        }

        Ok(expr)
    }

    fn logical(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let mut expr = Self::equality(parser)?;

        while parser.r#match(&[
            TokenKind::Keyword(KeywordKind::And),
            TokenKind::Keyword(KeywordKind::Or),
        ]) {
            let operator = LogicalOperator::try_from(parser.previous())?;
            let right = Self::equality(parser)?;
            expr = Expr::Binary {
                id: parser.assign_id(),
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

    fn equality(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let mut expr = Self::comparison(parser)?;

        while parser.r#match(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = BinaryOperator::try_from(parser.previous())?;
            let right = Self::comparison(parser)?;
            expr = Expr::Binary {
                id: parser.assign_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let mut expr = Self::term(parser)?;

        while parser.r#match(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = BinaryOperator::try_from(parser.previous())?;
            let right = Self::term(parser)?;
            expr = Expr::Binary {
                id: parser.assign_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let mut expr = Self::factor(parser)?;

        while parser.r#match(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = BinaryOperator::try_from(parser.previous())?;
            let right = Self::factor(parser)?;
            expr = Expr::Binary {
                id: parser.assign_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let mut expr = Self::unary(parser)?;

        while parser.r#match(&[TokenKind::Slash, TokenKind::Asterisk]) {
            let operator = BinaryOperator::try_from(parser.previous())?;
            let right = Self::unary(parser)?;
            expr = Expr::Binary {
                id: parser.assign_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        if parser.r#match(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = UnaryOperator::try_from(parser.previous())?;
            let right = Self::unary(parser)?;
            return Ok(Expr::Unary {
                id: parser.assign_id(),
                operator,
                right: Box::new(right),
            });
        }

        Self::member_access(parser)
    }

    fn member_access(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let mut expr = Self::call(parser)?;

        if parser.r#match(&[TokenKind::Dot]) {
            let mut members = Vec::new();

            loop {
                let member = Self::call(parser)?;
                members.push(member);

                if !parser.r#match(&[TokenKind::Dot]) {
                    break;
                }
            }
            expr = Expr::MemberAccess {
                id: parser.assign_id(),
                left: Box::new(expr),
                members,
            };
        }

        Ok(expr)
    }

    fn call(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let mut expr = Self::primary(parser)?;

        if parser.peek() == &TokenKind::LeftParen {
            let arguments = parser.punctuated(
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::Comma,
                |parser| Expr::parse(parser),
            )?;
            expr = Expr::Call {
                id: parser.assign_id(),
                callee: Box::new(expr),
                arguments,
            };
        }

        Ok(expr)
    }

    fn primary(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        if parser.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }

        match parser.peek() {
            // Literals
            TokenKind::String(_)
            | TokenKind::Number(_)
            | TokenKind::Keyword(KeywordKind::True | KeywordKind::False | KeywordKind::None) => {
                let literal = Literal::parse(parser)?;
                Ok(Expr::Primary {
                    id: parser.assign_id(),
                    value: Primary::Literal(literal),
                })
            }

            // Primitives
            TokenKind::LeftParen
            | TokenKind::LeftBrace
            | TokenKind::Identifier(_)
            | TokenKind::Keyword(KeywordKind::Context) => {
                let primitive = Primitive::parse(parser)?;
                Ok(Expr::Primary {
                    id: parser.assign_id(),
                    value: Primary::Primitive(primitive),
                })
            }
            _ => Err(Error::UnhandledPrimaryExpression),
        }
    }
}

impl Parsable for Expr {
    fn parse<'src, 'tok>(parser: &mut Parser<'src, 'tok>) -> Result<Self, Error> {
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
        Parser,
    };

    #[test]
    fn parses_arithmetic_binary_expr() {
        let source = "1 + 2 * 3 / 4";
        let tokens = Lexer::tokenize(source);
        let expr = Parser::parse_ast::<Expr>(&tokens).expect("Failed to parse expression");

        let expected = Expr::Binary {
            id: 6,
            left: Box::new(Expr::Primary {
                id: 0,
                value: Primary::Literal(Literal::Number(1.0)),
            }),
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Binary {
                id: 5,
                left: Box::new(Expr::Binary {
                    id: 3,
                    left: Box::new(Expr::Primary {
                        id: 1,
                        value: Primary::Literal(Literal::Number(2.0)),
                    }),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(Expr::Primary {
                        id: 2,
                        value: Primary::Literal(Literal::Number(3.0)),
                    }),
                }),
                operator: BinaryOperator::Divide,
                right: Box::new(Expr::Primary {
                    id: 4,
                    value: Primary::Literal(Literal::Number(4.0)),
                }),
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_equality_expr() {
        let source = "10 == 20 != 30";
        let tokens = Lexer::tokenize(source);
        let expr = Parser::parse_ast::<Expr>(&tokens).expect("Failed to parse expression");

        let expected = Expr::Binary {
            id: 4,
            left: Box::new(Expr::Binary {
                id: 2,
                left: Box::new(Expr::Primary {
                    id: 0,
                    value: Primary::Literal(Literal::Number(10.0)),
                }),
                operator: BinaryOperator::EqualEqual,
                right: Box::new(Expr::Primary {
                    id: 1,
                    value: Primary::Literal(Literal::Number(20.0)),
                }),
            }),
            operator: BinaryOperator::NotEqual,
            right: Box::new(Expr::Primary {
                id: 3,
                value: Primary::Literal(Literal::Number(30.0)),
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_comparison_expr() {
        let source = "5 > 3 < 9 >= 2 <= 10";
        let tokens = Lexer::tokenize(source);
        let expr = Parser::parse_ast::<Expr>(&tokens).expect("Failed to parse expression");

        let expected = Expr::Binary {
            id: 8,
            left: Box::new(Expr::Binary {
                id: 6,
                left: Box::new(Expr::Binary {
                    id: 4,
                    left: Box::new(Expr::Binary {
                        id: 2,
                        left: Box::new(Expr::Primary {
                            id: 0,
                            value: Primary::Literal(Literal::Number(5.0)),
                        }),
                        operator: BinaryOperator::Greater,
                        right: Box::new(Expr::Primary {
                            id: 1,
                            value: Primary::Literal(Literal::Number(3.0)),
                        }),
                    }),
                    operator: BinaryOperator::Less,
                    right: Box::new(Expr::Primary {
                        id: 3,
                        value: Primary::Literal(Literal::Number(9.0)),
                    }),
                }),
                operator: BinaryOperator::GreaterEqual,
                right: Box::new(Expr::Primary {
                    id: 5,
                    value: Primary::Literal(Literal::Number(2.0)),
                }),
            }),
            operator: BinaryOperator::LessEqual,
            right: Box::new(Expr::Primary {
                id: 7,
                value: Primary::Literal(Literal::Number(10.0)),
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_call_expr() {
        let source = "func(arg1, arg2)";
        let tokens = Lexer::tokenize(source);
        let expr = Parser::parse_ast::<Expr>(&tokens).expect("Failed to parse expression");

        let expected = Expr::Call {
            id: 6,
            callee: Box::new(Expr::Primary {
                id: 1,
                value: Primary::Primitive(Primitive::Identifier {
                    id: 0,
                    name: "func".to_string(),
                }),
            }),
            arguments: vec![
                Expr::Primary {
                    id: 3,
                    value: Primary::Primitive(Primitive::Identifier {
                        id: 2,
                        name: "arg1".to_string(),
                    }),
                },
                Expr::Primary {
                    id: 5,
                    value: Primary::Primitive(Primitive::Identifier {
                        id: 4,
                        name: "arg2".to_string(),
                    }),
                },
            ],
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_member_access_expr() {
        let source = "obj.prop1.prop2";
        let tokens = Lexer::tokenize(source);
        let expr = Parser::parse_ast::<Expr>(&tokens).expect("Failed to parse expression");

        let expected = Expr::MemberAccess {
            id: 6,
            left: Box::new(Expr::Primary {
                id: 1,
                value: Primary::Primitive(Primitive::Identifier {
                    id: 0,
                    name: "obj".to_string(),
                }),
            }),
            members: vec![
                Expr::Primary {
                    id: 3,
                    value: Primary::Primitive(Primitive::Identifier {
                        id: 2,
                        name: "prop1".to_string(),
                    }),
                },
                Expr::Primary {
                    id: 5,
                    value: Primary::Primitive(Primitive::Identifier {
                        id: 4,
                        name: "prop2".to_string(),
                    }),
                },
            ],
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_unary_expr() {
        let source = "-!42";
        let tokens = Lexer::tokenize(source);
        let expr = Parser::parse_ast::<Expr>(&tokens).expect("Failed to parse expression");

        let expected = Expr::Unary {
            id: 2,
            operator: UnaryOperator::Negate,
            right: Box::new(Expr::Unary {
                id: 1,
                operator: UnaryOperator::Not,
                right: Box::new(Expr::Primary {
                    id: 0,
                    value: Primary::Literal(Literal::Number(42.0)),
                }),
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_logical_expr() {
        let source = "true and false or true";
        let tokens = Lexer::tokenize(source);
        let expr = Parser::parse_ast::<Expr>(&tokens).expect("Failed to parse expression");

        let expected = Expr::Binary {
            id: 4,
            left: Box::new(Expr::Binary {
                id: 2,
                left: Box::new(Expr::Primary {
                    id: 0,
                    value: Primary::Literal(Literal::Boolean(true)),
                }),
                operator: BinaryOperator::And,
                right: Box::new(Expr::Primary {
                    id: 1,
                    value: Primary::Literal(Literal::Boolean(false)),
                }),
            }),
            operator: BinaryOperator::Or,
            right: Box::new(Expr::Primary {
                id: 3,
                value: Primary::Literal(Literal::Boolean(true)),
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_assignment_expr() {
        let source = "x = 10 + 20";
        let tokens = Lexer::tokenize(source);
        let expr = Parser::parse_ast::<Expr>(&tokens).expect("Failed to parse expression");

        let expected = Expr::Assignment {
            id: 5,
            name: Box::new(Expr::Primary {
                id: 1,
                value: Primary::Primitive(Primitive::Identifier {
                    id: 0,
                    name: "x".to_string(),
                }),
            }),
            value: Box::new(Expr::Binary {
                id: 4,
                left: Box::new(Expr::Primary {
                    id: 2,
                    value: Primary::Literal(Literal::Number(10.0)),
                }),
                operator: BinaryOperator::Add,
                right: Box::new(Expr::Primary {
                    id: 3,
                    value: Primary::Literal(Literal::Number(20.0)),
                }),
            }),
        };

        assert_eq!(expr, expected);
    }
}
