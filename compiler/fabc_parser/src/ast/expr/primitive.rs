use fabc_error::{kind::ErrorKind, Error};
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
    Context {
        info: NodeInfo,
    },
}

impl Parsable for Primitive {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        match parser.peek() {
            TokenKind::Identifier(_) => {
                let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;
                Ok(Primitive::Identifier {
                    info: NodeInfo {
                        id: parser.assign_id(),
                    },
                    name,
                })
            }
            TokenKind::Keyword(KeywordKind::Context) => {
                parser.consume(TokenKind::Keyword(KeywordKind::Context))?;
                Ok(Primitive::Context {
                    info: NodeInfo {
                        id: parser.assign_id(),
                    },
                })
            }
            TokenKind::LeftParen => {
                if let Some(closure) = parser.rollbacking(|parser| {
                    let params = parser.punctuated(
                        TokenKind::LeftParen,
                        TokenKind::RightParen,
                        TokenKind::Comma,
                        |parser| Primitive::parse(parser),
                    )?;

                    parser.consume(TokenKind::ArrowRight)?;

                    let body = Box::new(BlockStmt::parse(parser)?);

                    Ok(Primitive::Closure {
                        info: NodeInfo {
                            id: parser.assign_id(),
                        },
                        params,
                        body: *body,
                    })
                }) {
                    Ok(closure)
                } else {
                    let expr =
                        parser.enclosed(TokenKind::LeftParen, TokenKind::RightParen, |parser| {
                            Expr::parse(parser)
                        })?;
                    Ok(Primitive::Grouping {
                        info: NodeInfo {
                            id: parser.assign_id(),
                        },
                        expr: Box::new(expr),
                    })
                }
            }
            TokenKind::LeftBrace => {
                let object = ObjectDecl::parse(parser)?;
                Ok(Primitive::Object {
                    info: NodeInfo {
                        id: parser.assign_id(),
                    },
                    value: object,
                })
            }
            _ => Err(Error::new(
                ErrorKind::UnrecognizedPrimitive {
                    primitive: parser.peek().to_string(),
                },
                parser.current_token(),
            )),
        }
    }
}

#[cfg(test)]
mod primitive_tests {
    use std::collections::HashMap;

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
                info: NodeInfo { id: 0 },
                name: "foo".to_string(),
            }
        );

        let source = "(x)";
        let tokens = Lexer::tokenize(source);
        let primitive = Parser::parse_ast::<Primitive>(&tokens).expect("Failed to parse primitive");
        assert_eq!(
            primitive,
            Primitive::Grouping {
                info: NodeInfo { id: 2 },
                expr: Box::new(Expr::Primary {
                    info: NodeInfo { id: 1 },
                    value: Primary::Primitive(Primitive::Identifier {
                        info: NodeInfo { id: 0 },
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
                info: NodeInfo { id: 0 }
            }
        );
    }

    #[test]
    fn parses_object_primitive() {
        let source = "{ key1: 42, key2: true }";
        let tokens = Lexer::tokenize(source);
        let primitive = Parser::parse_ast::<Primitive>(&tokens).expect("Failed to parse primitive");

        let expected = Primitive::Object {
            info: NodeInfo { id: 3 },
            value: ObjectDecl {
                info: NodeInfo { id: 2 },
                map: {
                    let mut map = HashMap::new();
                    map.insert(
                        "key1".to_string(),
                        Expr::Primary {
                            info: NodeInfo { id: 0 },
                            value: Primary::Literal(Literal::Number(42.0)),
                        },
                    );
                    map.insert(
                        "key2".to_string(),
                        Expr::Primary {
                            info: NodeInfo { id: 1 },
                            value: Primary::Literal(Literal::Boolean(true)),
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
            info: NodeInfo { id: 9 },
            params: vec![
                Primitive::Identifier {
                    info: NodeInfo { id: 0 },
                    name: "x".to_string(),
                },
                Primitive::Identifier {
                    info: NodeInfo { id: 1 },
                    name: "y".to_string(),
                },
            ],
            body: BlockStmt {
                info: NodeInfo { id: 8 },
                statements: vec![Stmt::Expr(ExprStmt {
                    info: NodeInfo { id: 7 },
                    expr: Expr::Binary {
                        info: NodeInfo { id: 6 },
                        left: Box::new(Expr::Primary {
                            info: NodeInfo { id: 3 },
                            value: Primary::Primitive(Primitive::Identifier {
                                info: NodeInfo { id: 2 },
                                name: "x".to_string(),
                            }),
                        }),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expr::Primary {
                            info: NodeInfo { id: 5 },
                            value: Primary::Primitive(Primitive::Identifier {
                                info: NodeInfo { id: 4 },
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
