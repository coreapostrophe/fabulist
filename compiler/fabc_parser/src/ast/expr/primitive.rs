use std::collections::HashMap;

use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{
        decl::object::ObjectDecl,
        expr::{Expr, Primary},
        stmt::block::BlockStmt,
    },
    error::Error,
    expect_token, Parsable,
};

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Identifier(String),
    Grouping(Box<Expr>),
    Object(HashMap<String, Expr>),
    Closure {
        params: Vec<String>,
        body: BlockStmt,
    },
    Context,
}

impl Parsable for Primitive {
    fn parse(parser: &mut crate::Parser) -> Result<Self, Error> {
        if parser.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }

        match parser.peek() {
            TokenKind::Identifier(_) => {
                let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;
                Ok(Primitive::Identifier(name))
            }
            TokenKind::Keyword(KeywordKind::Context) => {
                parser.consume(TokenKind::Keyword(KeywordKind::Context))?;
                Ok(Primitive::Context)
            }
            TokenKind::LeftParen => {
                if let Some(closure) = parser.rollbacking(|parser| {
                    let expr_tuple = parser.punctuated(
                        TokenKind::LeftParen,
                        TokenKind::RightParen,
                        TokenKind::Comma,
                        |parser| Expr::parse(parser),
                    )?;

                    parser.consume(TokenKind::ArrowRight)?;

                    let body = Box::new(BlockStmt::parse(parser)?);

                    let params = expr_tuple
                        .into_iter()
                        .map(|expr| match expr {
                            Expr::Primary(Primary::Primitive(Primitive::Identifier(name))) => {
                                Ok(name)
                            }
                            _ => Err(Error::ExpectedFound {
                                expected: "identifier".to_string(),
                                found: format!("{:?}", expr),
                            }),
                        })
                        .collect::<Result<Vec<String>, Error>>()?;

                    Ok(Primitive::Closure {
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
                    Ok(Primitive::Grouping(Box::new(expr)))
                }
            }
            TokenKind::LeftBrace => {
                let map = ObjectDecl::parse(parser)?.map;
                Ok(Primitive::Object(map))
            }
            _ => Err(Error::UnhandledPrimitive),
        }
    }
}

#[cfg(test)]
mod primitive_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, primitive::Primitive, BinaryOperator, Expr, Primary},
            stmt::{block::BlockStmt, expr::ExprStmt, Stmt},
        },
        Parser,
    };

    #[test]
    fn parses_basic_primitives() {
        let source = "foo";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let primitive = Parser::parse::<Primitive>(&tokens).expect("Failed to parse primitive");
        assert_eq!(primitive, Primitive::Identifier("foo".to_string()));

        let source = "(x)";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let primitive = Parser::parse::<Primitive>(&tokens).expect("Failed to parse primitive");
        assert_eq!(
            primitive,
            Primitive::Grouping(Box::new(Expr::Primary(Primary::Primitive(
                Primitive::Identifier("x".to_string())
            ))))
        );

        let source = "context";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let primitive = Parser::parse::<Primitive>(&tokens).expect("Failed to parse primitive");
        assert_eq!(primitive, Primitive::Context);
    }

    #[test]
    fn parses_object_primitive() {
        let source = "{ key1: 42, key2: true }";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let primitive = Parser::parse::<Primitive>(&tokens).expect("Failed to parse primitive");

        let expected = Primitive::Object({
            let mut map = HashMap::new();
            map.insert(
                "key1".to_string(),
                Expr::Primary(Primary::Literal(Literal::Number(42.0))),
            );
            map.insert(
                "key2".to_string(),
                Expr::Primary(Primary::Literal(Literal::Boolean(true))),
            );
            map
        });
        assert_eq!(primitive, expected);
    }

    #[test]
    fn parses_closure_primitive() {
        let source = "(x, y) => { x + y; }";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let primitive = Parser::parse::<Primitive>(&tokens).expect("Failed to parse primitive");

        let expected = Primitive::Closure {
            params: vec!["x".to_string(), "y".to_string()],
            body: BlockStmt {
                statements: vec![Stmt::Expr(ExprStmt {
                    expr: Expr::Binary {
                        left: Box::new(Expr::Primary(Primary::Primitive(Primitive::Identifier(
                            "x".to_string(),
                        )))),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expr::Primary(Primary::Primitive(Primitive::Identifier(
                            "y".to_string(),
                        )))),
                    },
                })],
            },
        };
        assert_eq!(primitive, expected);
    }
}
