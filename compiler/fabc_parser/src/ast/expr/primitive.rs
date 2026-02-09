use fabc_error::{kind::CompileErrorKind, Error, Span};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{decl::object::ObjectDecl, expr::Expr, stmt::block::BlockStmt, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Identifier {
        info: NodeInfo,
        name: String,
    },
    Grouping {
        info: NodeInfo,
        expr: Box<Expr>,
    },
    Object {
        info: NodeInfo,
        value: ObjectDecl,
    },
    Closure {
        info: NodeInfo,
        params: Vec<Primitive>,
        body: BlockStmt,
    },
    StoryIdentifier {
        info: NodeInfo,
        name: String,
    },
    Context {
        info: NodeInfo,
    },
}

impl Primitive {
    pub fn info(&self) -> &NodeInfo {
        match self {
            Primitive::Identifier { info, .. } => info,
            Primitive::Grouping { info, .. } => info,
            Primitive::Object { info, .. } => info,
            Primitive::Closure { info, .. } => info,
            Primitive::StoryIdentifier { info, .. } => info,
            Primitive::Context { info } => info,
        }
    }
}

impl Parsable for Primitive {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        match parser.peek() {
            TokenKind::Commat => {
                let start_span = parser.start_span();
                parser.consume(TokenKind::Commat)?;
                let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;
                let end_span = parser.end_span();

                Ok(Primitive::StoryIdentifier {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from((start_span, end_span)),
                    },
                    name,
                })
            }
            TokenKind::Identifier(_) => {
                let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;

                Ok(Primitive::Identifier {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from(parser.previous_token()),
                    },
                    name,
                })
            }
            TokenKind::Keyword(KeywordKind::Context) => {
                parser.consume(TokenKind::Keyword(KeywordKind::Context))?;

                Ok(Primitive::Context {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from(parser.previous_token()),
                    },
                })
            }
            TokenKind::LeftParen => {
                if let Some(closure) = parser.rollbacking(|parser| {
                    let start_span = parser.start_span();
                    let params = parser.punctuated(
                        TokenKind::LeftParen,
                        TokenKind::RightParen,
                        TokenKind::Comma,
                        Primitive::parse,
                    )?;
                    parser.consume(TokenKind::ArrowRight)?;
                    let body = BlockStmt::parse(parser)?;
                    let end_span = parser.end_span();

                    Ok(Primitive::Closure {
                        info: NodeInfo {
                            id: parser.assign_id(),
                            span: Span::from((start_span, end_span)),
                        },
                        params,
                        body,
                    })
                }) {
                    Ok(closure)
                } else {
                    let start_span = parser.start_span();
                    let expr = parser.enclosed(
                        TokenKind::LeftParen,
                        TokenKind::RightParen,
                        Expr::parse,
                    )?;
                    let end_span = parser.end_span();

                    Ok(Primitive::Grouping {
                        info: NodeInfo {
                            id: parser.assign_id(),
                            span: Span::from((start_span, end_span)),
                        },
                        expr: Box::new(expr),
                    })
                }
            }
            TokenKind::LeftBrace => {
                let start_span = parser.start_span();
                let object = ObjectDecl::parse(parser)?;
                let end_span = parser.end_span();

                Ok(Primitive::Object {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from((start_span, end_span)),
                    },
                    value: object,
                })
            }
            _ => Err(Error::new(
                CompileErrorKind::UnrecognizedPrimitive {
                    primitive: parser.peek().to_string(),
                },
                parser.peek_token(),
            )),
        }
    }
}

#[cfg(test)]
mod primitive_tests {
    use std::collections::HashMap;

    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::object::ObjectDecl,
            expr::{literal::Literal, primitive::Primitive, BinaryOperator, Expr, Primary},
            stmt::{block::BlockStmt, expr::ExprStmt, Stmt},
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_basic_primitives() {
        let source = "foo";
        let tokens = Lexer::tokenize(source);
        let primitive = Parser::parse_ast::<Primitive>(&tokens).expect("Failed to parse primitive");
        assert_eq!(
            primitive,
            Primitive::Identifier {
                info: NodeInfo {
                    id: 0,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 3)))
                },
                name: "foo".to_string(),
            }
        );

        let source = "@foo";
        let tokens = Lexer::tokenize(source);
        let primitive = Parser::parse_ast::<Primitive>(&tokens).expect("Failed to parse primitive");
        assert_eq!(
            primitive,
            Primitive::StoryIdentifier {
                info: NodeInfo {
                    id: 0,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 4)))
                },
                name: "foo".to_string(),
            }
        );

        let source = "(x)";
        let tokens = Lexer::tokenize(source);
        let primitive = Parser::parse_ast::<Primitive>(&tokens).expect("Failed to parse primitive");
        assert_eq!(
            primitive,
            Primitive::Grouping {
                info: NodeInfo {
                    id: 2,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 3)))
                },
                expr: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(1, 2), LineCol::new(1, 2))),
                    },
                    value: Primary::Primitive(Primitive::Identifier {
                        info: NodeInfo {
                            id: 0,
                            span: Span::from((LineCol::new(1, 2), LineCol::new(1, 2)))
                        },
                        name: "x".to_string(),
                    }),
                }),
            }
        );

        let source = "context";
        let tokens = Lexer::tokenize(source);
        let primitive = Parser::parse_ast::<Primitive>(&tokens).expect("Failed to parse primitive");
        assert_eq!(
            primitive,
            Primitive::Context {
                info: NodeInfo {
                    id: 0,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 7)))
                }
            }
        );
    }

    #[test]
    fn parses_object_primitive() {
        let source = "{ key1: 42, key2: true }";
        let tokens = Lexer::tokenize(source);
        let primitive = Parser::parse_ast::<Primitive>(&tokens).expect("Failed to parse primitive");

        let expected = Primitive::Object {
            info: NodeInfo {
                id: 5,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 24))),
            },
            value: ObjectDecl {
                info: NodeInfo {
                    id: 4,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 24))),
                },
                map: {
                    let mut map = HashMap::new();
                    map.insert(
                        "key1".to_string(),
                        Expr::Primary {
                            info: NodeInfo {
                                id: 1,
                                span: Span::from((LineCol::new(1, 9), LineCol::new(1, 10))),
                            },
                            value: Primary::Literal(Literal::Number {
                                info: NodeInfo {
                                    id: 0,
                                    span: Span::from((LineCol::new(1, 9), LineCol::new(1, 10))),
                                },
                                value: 42.0,
                            }),
                        },
                    );
                    map.insert(
                        "key2".to_string(),
                        Expr::Primary {
                            info: NodeInfo {
                                id: 3,
                                span: Span::from((LineCol::new(1, 19), LineCol::new(1, 22))),
                            },
                            value: Primary::Literal(Literal::Boolean {
                                info: NodeInfo {
                                    id: 2,
                                    span: Span::from((LineCol::new(1, 19), LineCol::new(1, 22))),
                                },
                                value: true,
                            }),
                        },
                    );
                    map
                },
            },
        };
        assert_eq!(primitive, expected);
    }

    #[test]
    fn parses_closure_primitive() {
        let source = "(x, y) => { x + y; }";
        let tokens = Lexer::tokenize(source);
        let primitive = Parser::parse_ast::<Primitive>(&tokens).expect("Failed to parse primitive");

        let expected = Primitive::Closure {
            info: NodeInfo {
                id: 9,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 20))),
            },
            params: vec![
                Primitive::Identifier {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 2), LineCol::new(1, 2))),
                    },
                    name: "x".to_string(),
                },
                Primitive::Identifier {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(1, 5), LineCol::new(1, 5))),
                    },
                    name: "y".to_string(),
                },
            ],
            body: BlockStmt {
                info: NodeInfo {
                    id: 8,
                    span: Span::from((LineCol::new(1, 11), LineCol::new(1, 20))),
                },
                first_return: None,
                statements: vec![Stmt::Expr(ExprStmt {
                    info: NodeInfo {
                        id: 7,
                        span: Span::from((LineCol::new(1, 13), LineCol::new(1, 18))),
                    },
                    expr: Expr::Binary {
                        info: NodeInfo {
                            id: 6,
                            span: Span::from((LineCol::new(1, 13), LineCol::new(1, 17))),
                        },
                        left: Box::new(Expr::Primary {
                            info: NodeInfo {
                                id: 3,
                                span: Span::from((LineCol::new(1, 13), LineCol::new(1, 13))),
                            },
                            value: Primary::Primitive(Primitive::Identifier {
                                info: NodeInfo {
                                    id: 2,
                                    span: Span::from((LineCol::new(1, 13), LineCol::new(1, 13))),
                                },
                                name: "x".to_string(),
                            }),
                        }),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expr::Primary {
                            info: NodeInfo {
                                id: 5,
                                span: Span::from((LineCol::new(1, 17), LineCol::new(1, 17))),
                            },
                            value: Primary::Primitive(Primitive::Identifier {
                                info: NodeInfo {
                                    id: 4,
                                    span: Span::from((LineCol::new(1, 17), LineCol::new(1, 17))),
                                },
                                name: "y".to_string(),
                            }),
                        }),
                    },
                })],
            },
        };
        assert_eq!(primitive, expected);
    }
}
