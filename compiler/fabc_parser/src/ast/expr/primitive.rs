use std::collections::HashMap;

use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{
    ast::{decl::parameter_body::ParameterBodyDecl, expr::Expr, stmt::block::BlockStmt},
    error::Error,
    Parsable,
};

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Identifier(String),
    Path(Vec<String>),
    Grouping(Box<Expr>),
    Object(HashMap<String, Expr>),
    Closure {
        params: ParameterBodyDecl,
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
                let name = match parser.advance() {
                    Token::Identifier(ident) => ident.to_string(),
                    _ => {
                        return Err(Error::ExpectedFound {
                            expected: "identifier".to_string(),
                            found: parser.peek().to_string(),
                        })
                    }
                };
                Ok(Primitive::Identifier(name.clone()))
            }
            Token::Path(_) => {
                let segments = match parser.advance() {
                    Token::Path(segments) => segments.clone(),
                    _ => {
                        return Err(Error::ExpectedFound {
                            expected: "path".to_string(),
                            found: parser.peek().to_string(),
                        })
                    }
                };
                Ok(Primitive::Path(segments.clone()))
            }
            Token::Keyword(KeywordKind::Context) => {
                parser.consume(Token::Keyword(KeywordKind::Context))?;
                Ok(Primitive::Context)
            }
            Token::LeftParen => {
                parser.consume(Token::LeftParen)?;
                let expr = Expr::parse(parser)?;
                parser.consume(Token::RightParen)?;
                Ok(Primitive::Grouping(Box::new(expr)))
            }
            Token::LeftBrace => {
                parser.consume(Token::LeftBrace)?;

                let mut map = HashMap::new();
                while parser.peek() != &Token::RightBrace {
                    let key = match parser.advance() {
                        Token::Identifier(ident) => ident.to_string(),
                        _ => {
                            return Err(Error::ExpectedFound {
                                expected: "identifier".to_string(),
                                found: parser.peek().to_string(),
                            })
                        }
                    };

                    parser.consume(Token::Colon)?;

                    let value = Expr::parse(parser)?;
                    map.insert(key, value);

                    if !parser.r#match(vec![Token::Comma]) {
                        break;
                    }
                }

                parser.consume(Token::RightBrace)?;

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
        ast::expr::{literal::Literal, primitive::Primitive, Expr, Primary},
        Parsable, Parser,
    };

    #[test]
    fn parses_primitives() {
        let tokens = vec![Token::Identifier("foo".to_string())];
        let mut parser = Parser::new(&tokens);
        assert_eq!(
            Primitive::parse(&mut parser).unwrap(),
            Primitive::Identifier("foo".to_string())
        );

        let tokens = vec![Token::Path(vec![
            "module".to_string(),
            "symbol".to_string(),
        ])];
        let mut parser = Parser::new(&tokens);
        assert_eq!(
            Primitive::parse(&mut parser).unwrap(),
            Primitive::Path(vec!["module".to_string(), "symbol".to_string()])
        );

        let tokens = vec![
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen,
        ];
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

        let mut parser = Parser::new(tokens);
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
}
