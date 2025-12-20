use std::collections::HashMap;

use fabc_lexer::{keywords::KeywordKind, tokens::Token};

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
            Token::Identifier(_) => {
                let name = expect_token!(parser, Token::Identifier, "identifier")?;
                Ok(Primitive::Identifier(name))
            }
            Token::Keyword(KeywordKind::Context) => {
                parser.consume(Token::Keyword(KeywordKind::Context))?;
                Ok(Primitive::Context)
            }
            Token::LeftParen => {
                if let Some(closure) = parser.rollbacking(|parser| {
                    let expr_tuple = parser.punctuated(
                        Token::LeftParen,
                        Token::RightParen,
                        Token::Comma,
                        |parser| Expr::parse(parser),
                    )?;

                    parser.consume(Token::ArrowRight)?;

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
                    let expr = parser.enclosed(Token::LeftParen, Token::RightParen, |parser| {
                        Expr::parse(parser)
                    })?;
                    Ok(Primitive::Grouping(Box::new(expr)))
                }
            }
            Token::LeftBrace => {
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

    use fabc_lexer::{keywords::KeywordKind, tokens::Token};

    use crate::{
        ast::{
            expr::{literal::Literal, primitive::Primitive, BinaryOperator, Expr, Primary},
            stmt::{block::BlockStmt, expr::ExprStmt, Stmt},
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_basic_primitives() {
        let tokens = vec![Token::Identifier("foo")];
        let mut parser = Parser::new(&tokens);
        assert_eq!(
            Primitive::parse(&mut parser).unwrap(),
            Primitive::Identifier("foo".to_string())
        );

        let tokens = vec![Token::LeftParen, Token::Identifier("x"), Token::RightParen];
        let mut parser = Parser::new(&tokens);
        assert_eq!(
            Primitive::parse(&mut parser).unwrap(),
            Primitive::Grouping(Box::new(Expr::Primary(Primary::Primitive(
                Primitive::Identifier("x".to_string())
            ))))
        );

        let tokens = vec![Token::Keyword(KeywordKind::Context)];
        let mut parser = Parser::new(&tokens);
        assert_eq!(Primitive::parse(&mut parser).unwrap(), Primitive::Context);
    }

    #[test]
    fn parses_object_primitive() {
        let source = "{ key1: 42, key2: true }";
        let mut lexer = fabc_lexer::Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(&tokens);
        let primitive = Primitive::parse(&mut parser).expect("Failed to parse");

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
        let mut lexer = fabc_lexer::Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(&tokens);
        let primitive = Primitive::parse(&mut parser).expect("Failed to parse");

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
