use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{ast::expr::Expr, error::Error, Parsable};

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Identifier(String),
    Path(Vec<String>),
    Grouping(Box<Expr>),
    Context,
}

impl Parsable for Primitive {
    fn parse(parser: &mut crate::Parser) -> Result<Self, Error> {
        if parser.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }

        match parser.advance() {
            Token::Identifier(name) => Ok(Primitive::Identifier(name.clone())),
            Token::Path(segments) => Ok(Primitive::Path(segments.clone())),
            Token::Keyword(KeywordKind::Context) => Ok(Primitive::Context),
            Token::LeftParen => {
                let expr = Expr::parse(parser)?;
                parser.consume(Token::RightParen)?;
                Ok(Primitive::Grouping(Box::new(expr)))
            }
            _ => Err(Error::UnhandledPrimitive),
        }
    }
}

#[cfg(test)]
mod primitive_tests {
    use fabc_lexer::{keywords::KeywordKind, tokens::Token};

    use crate::{
        ast::expr::{primitive::Primitive, Expr, Primary},
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
}
