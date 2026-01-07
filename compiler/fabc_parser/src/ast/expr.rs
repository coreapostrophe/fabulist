use fabc_error::{kind::ErrorKind, Error, LineCol, Span};
use fabc_lexer::{
    keywords::KeywordKind,
    tokens::{Token, TokenKind},
};

use crate::{
    ast::{
        expr::{literal::Literal, primitive::Primitive},
        NodeInfo,
    },
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

impl TryFrom<&Token<'_>> for BinaryOperator {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        match token.kind {
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
            _ => Err(Error::new(
                ErrorKind::InvalidOperator {
                    operator: token.kind.to_string(),
                },
                token,
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
}

impl TryFrom<&Token<'_>> for UnaryOperator {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        match token.kind {
            TokenKind::Bang => Ok(UnaryOperator::Not),
            TokenKind::Minus => Ok(UnaryOperator::Negate),
            _ => Err(Error::new(
                ErrorKind::InvalidOperator {
                    operator: token.kind.to_string(),
                },
                token,
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl TryFrom<&Token<'_>> for LogicalOperator {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        match token.kind {
            TokenKind::Keyword(KeywordKind::And) => Ok(LogicalOperator::And),
            TokenKind::Keyword(KeywordKind::Or) => Ok(LogicalOperator::Or),
            _ => Err(Error::new(
                ErrorKind::InvalidOperator {
                    operator: token.kind.to_string(),
                },
                token,
            )),
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
        info: NodeInfo,
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Unary {
        info: NodeInfo,
        operator: UnaryOperator,
        right: Box<Expr>,
    },
    Assignment {
        info: NodeInfo,
        name: Box<Expr>,
        value: Box<Expr>,
    },
    MemberAccess {
        info: NodeInfo,
        left: Box<Expr>,
        members: Vec<Expr>,
    },
    Call {
        info: NodeInfo,
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Primary {
        info: NodeInfo,
        value: Primary,
    },
    Grouping {
        info: NodeInfo,
        expression: Box<Expr>,
    },
}

impl Expr {
    pub fn assignment(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::logical(parser)?;

        if parser.r#match(&[TokenKind::Equal]) {
            let value = Self::assignment(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Assignment {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                name: Box::new(expr),
                value: Box::new(value),
            }
        }

        Ok(expr)
    }

    fn logical(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::equality(parser)?;

        while parser.r#match(&[
            TokenKind::Keyword(KeywordKind::And),
            TokenKind::Keyword(KeywordKind::Or),
        ]) {
            let operator = LogicalOperator::try_from(parser.previous_token())?;
            let right = Self::equality(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
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
        let start_span = parser.start_span();
        let mut expr = Self::comparison(parser)?;

        while parser.r#match(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = BinaryOperator::try_from(parser.previous_token())?;
            let right = Self::comparison(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::term(parser)?;

        while parser.r#match(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = BinaryOperator::try_from(parser.previous_token())?;
            let right = Self::term(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::factor(parser)?;

        while parser.r#match(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = BinaryOperator::try_from(parser.previous_token())?;
            let right = Self::factor(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = LineCol::from_token(parser.current_token());
        let mut expr = Self::unary(parser)?;

        while parser.r#match(&[TokenKind::Slash, TokenKind::Asterisk]) {
            let operator = BinaryOperator::try_from(parser.previous_token())?;
            let right = Self::unary(parser)?;
            let end_span = LineCol::from_token(parser.previous_token());

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        if parser.r#match(&[TokenKind::Bang, TokenKind::Minus]) {
            let start_span = LineCol::from_token(parser.current_token());
            let operator = UnaryOperator::try_from(parser.previous_token())?;
            let right = Self::unary(parser)?;
            let end_span = LineCol::from_token(parser.previous_token());

            return Ok(Expr::Unary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                operator,
                right: Box::new(right),
            });
        }

        Self::member_access(parser)
    }

    fn member_access(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
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
            let end_span = parser.end_span();

            expr = Expr::MemberAccess {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                members,
            };
        }

        Ok(expr)
    }

    fn call(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::primary(parser)?;

        if parser.peek() == &TokenKind::LeftParen {
            let arguments = parser.punctuated(
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::Comma,
                |parser| Expr::parse(parser),
            )?;
            let end_span = parser.end_span();

            expr = Expr::Call {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                callee: Box::new(expr),
                arguments,
            };
        }

        Ok(expr)
    }

    fn primary(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        match parser.peek() {
            // Literals
            TokenKind::String(_)
            | TokenKind::Number(_)
            | TokenKind::Keyword(KeywordKind::True | KeywordKind::False | KeywordKind::None) => {
                let literal = Literal::parse(parser)?;

                Ok(Expr::Primary {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from(parser.previous_token()),
                    },
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
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from(parser.previous_token()),
                    },
                    value: Primary::Primitive(primitive),
                })
            }
            _ => Err(Error::new(
                ErrorKind::UnrecognizedPrimary {
                    primary: parser.peek().to_string(),
                },
                parser.current_token(),
            )),
        }
    }
}

impl Parsable for Expr {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        Self::assignment(parser)
    }
}

#[cfg(test)]
mod expr_tests {
    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{
                literal::Literal, primitive::Primitive, BinaryOperator, Expr, Primary,
                UnaryOperator,
            },
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_arithmetic_binary_expr() {
        let source = "1 + 2 * 3 / 4";
        let tokens = Lexer::tokenize(source);
        let expr = Parser::parse_ast::<Expr>(&tokens).expect("Failed to parse expression");

        let expected = Expr::Binary {
            info: NodeInfo {
                id: 6,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 13))),
            },
            left: Box::new(Expr::Primary {
                info: NodeInfo {
                    id: 0,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 1))),
                },
                value: Primary::Literal(Literal::Number(1.0)),
            }),
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Binary {
                info: NodeInfo {
                    id: 5,
                    span: Span::from((LineCol::new(1, 5), LineCol::new(1, 13))),
                },
                left: Box::new(Expr::Binary {
                    info: NodeInfo {
                        id: 3,
                        span: Span::from((LineCol::new(1, 5), LineCol::new(1, 9))),
                    },
                    left: Box::new(Expr::Primary {
                        info: NodeInfo {
                            id: 1,
                            span: Span::from((LineCol::new(1, 5), LineCol::new(1, 5))),
                        },
                        value: Primary::Literal(Literal::Number(2.0)),
                    }),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(Expr::Primary {
                        info: NodeInfo {
                            id: 2,
                            span: Span::from((LineCol::new(1, 9), LineCol::new(1, 9))),
                        },
                        value: Primary::Literal(Literal::Number(3.0)),
                    }),
                }),
                operator: BinaryOperator::Divide,
                right: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 4,
                        span: Span::from((LineCol::new(1, 13), LineCol::new(1, 13))),
                    },
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
            info: NodeInfo {
                id: 4,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 14))),
            },
            left: Box::new(Expr::Binary {
                info: NodeInfo {
                    id: 2,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 8))),
                },
                left: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 1), LineCol::new(1, 2))),
                    },
                    value: Primary::Literal(Literal::Number(10.0)),
                }),
                operator: BinaryOperator::EqualEqual,
                right: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(1, 7), LineCol::new(1, 8))),
                    },
                    value: Primary::Literal(Literal::Number(20.0)),
                }),
            }),
            operator: BinaryOperator::NotEqual,
            right: Box::new(Expr::Primary {
                info: NodeInfo {
                    id: 3,
                    span: Span::from((LineCol::new(1, 13), LineCol::new(1, 14))),
                },
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
            info: NodeInfo {
                id: 8,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 20))),
            },
            left: Box::new(Expr::Binary {
                info: NodeInfo {
                    id: 6,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 14))),
                },
                left: Box::new(Expr::Binary {
                    info: NodeInfo {
                        id: 4,
                        span: Span::from((LineCol::new(1, 1), LineCol::new(1, 9))),
                    },
                    left: Box::new(Expr::Binary {
                        info: NodeInfo {
                            id: 2,
                            span: Span::from((LineCol::new(1, 1), LineCol::new(1, 5))),
                        },
                        left: Box::new(Expr::Primary {
                            info: NodeInfo {
                                id: 0,
                                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 1))),
                            },
                            value: Primary::Literal(Literal::Number(5.0)),
                        }),
                        operator: BinaryOperator::Greater,
                        right: Box::new(Expr::Primary {
                            info: NodeInfo {
                                id: 1,
                                span: Span::from((LineCol::new(1, 5), LineCol::new(1, 5))),
                            },
                            value: Primary::Literal(Literal::Number(3.0)),
                        }),
                    }),
                    operator: BinaryOperator::Less,
                    right: Box::new(Expr::Primary {
                        info: NodeInfo {
                            id: 3,
                            span: Span::from((LineCol::new(1, 9), LineCol::new(1, 9))),
                        },
                        value: Primary::Literal(Literal::Number(9.0)),
                    }),
                }),
                operator: BinaryOperator::GreaterEqual,
                right: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 5,
                        span: Span::from((LineCol::new(1, 14), LineCol::new(1, 14))),
                    },
                    value: Primary::Literal(Literal::Number(2.0)),
                }),
            }),
            operator: BinaryOperator::LessEqual,
            right: Box::new(Expr::Primary {
                info: NodeInfo {
                    id: 7,
                    span: Span::from((LineCol::new(1, 19), LineCol::new(1, 20))),
                },
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
            info: NodeInfo {
                id: 6,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 16))),
            },
            callee: Box::new(Expr::Primary {
                info: NodeInfo {
                    id: 1,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 4))),
                },
                value: Primary::Primitive(Primitive::Identifier {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 1), LineCol::new(1, 4))),
                    },
                    name: "func".to_string(),
                }),
            }),
            arguments: vec![
                Expr::Primary {
                    info: NodeInfo {
                        id: 3,
                        span: Span::from((LineCol::new(1, 6), LineCol::new(1, 9))),
                    },
                    value: Primary::Primitive(Primitive::Identifier {
                        info: NodeInfo {
                            id: 2,
                            span: Span::from((LineCol::new(1, 6), LineCol::new(1, 9))),
                        },
                        name: "arg1".to_string(),
                    }),
                },
                Expr::Primary {
                    info: NodeInfo {
                        id: 5,
                        span: Span::from((LineCol::new(1, 12), LineCol::new(1, 15))),
                    },
                    value: Primary::Primitive(Primitive::Identifier {
                        info: NodeInfo {
                            id: 4,
                            span: Span::from((LineCol::new(1, 12), LineCol::new(1, 15))),
                        },
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
            info: NodeInfo {
                id: 6,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 15))),
            },
            left: Box::new(Expr::Primary {
                info: NodeInfo {
                    id: 1,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 3))),
                },
                value: Primary::Primitive(Primitive::Identifier {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 1), LineCol::new(1, 3))),
                    },
                    name: "obj".to_string(),
                }),
            }),
            members: vec![
                Expr::Primary {
                    info: NodeInfo {
                        id: 3,
                        span: Span::from((LineCol::new(1, 5), LineCol::new(1, 9))),
                    },
                    value: Primary::Primitive(Primitive::Identifier {
                        info: NodeInfo {
                            id: 2,
                            span: Span::from((LineCol::new(1, 5), LineCol::new(1, 9))),
                        },
                        name: "prop1".to_string(),
                    }),
                },
                Expr::Primary {
                    info: NodeInfo {
                        id: 5,
                        span: Span::from((LineCol::new(1, 11), LineCol::new(1, 15))),
                    },
                    value: Primary::Primitive(Primitive::Identifier {
                        info: NodeInfo {
                            id: 4,
                            span: Span::from((LineCol::new(1, 11), LineCol::new(1, 15))),
                        },
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
            info: NodeInfo {
                id: 2,
                span: Span::from((LineCol::new(1, 2), LineCol::new(1, 3))),
            },
            operator: UnaryOperator::Negate,
            right: Box::new(Expr::Unary {
                info: NodeInfo {
                    id: 1,
                    span: Span::from((LineCol::new(1, 3), LineCol::new(1, 3))),
                },
                operator: UnaryOperator::Not,
                right: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 3), LineCol::new(1, 4))),
                    },
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
            info: NodeInfo {
                id: 4,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 22))),
            },
            left: Box::new(Expr::Binary {
                info: NodeInfo {
                    id: 2,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 14))),
                },
                left: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 1), LineCol::new(1, 4))),
                    },
                    value: Primary::Literal(Literal::Boolean(true)),
                }),
                operator: BinaryOperator::And,
                right: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(1, 10), LineCol::new(1, 14))),
                    },
                    value: Primary::Literal(Literal::Boolean(false)),
                }),
            }),
            operator: BinaryOperator::Or,
            right: Box::new(Expr::Primary {
                info: NodeInfo {
                    id: 3,
                    span: Span::from((LineCol::new(1, 19), LineCol::new(1, 22))),
                },
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
            info: NodeInfo {
                id: 5,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 11))),
            },
            name: Box::new(Expr::Primary {
                info: NodeInfo {
                    id: 1,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 1))),
                },
                value: Primary::Primitive(Primitive::Identifier {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 1), LineCol::new(1, 1))),
                    },
                    name: "x".to_string(),
                }),
            }),
            value: Box::new(Expr::Binary {
                info: NodeInfo {
                    id: 4,
                    span: Span::from((LineCol::new(1, 5), LineCol::new(1, 11))),
                },
                left: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 2,
                        span: Span::from((LineCol::new(1, 5), LineCol::new(1, 6))),
                    },
                    value: Primary::Literal(Literal::Number(10.0)),
                }),
                operator: BinaryOperator::Add,
                right: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 3,
                        span: Span::from((LineCol::new(1, 10), LineCol::new(1, 11))),
                    },
                    value: Primary::Literal(Literal::Number(20.0)),
                }),
            }),
        };

        assert_eq!(expr, expected);
    }
}
