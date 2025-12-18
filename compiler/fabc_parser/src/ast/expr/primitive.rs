use fabc_lexer::tokens::Token;

use crate::{error::Error, Parsable};

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Identifier(String),
    Path(Vec<String>),
}

impl Parsable for Primitive {
    fn parse(parser: &mut crate::Parser) -> Result<Self, Error> {
        let token = parser.advance();

        match token {
            Token::Identifier(name) => Ok(Primitive::Identifier(name.clone())),
            Token::Path(segments) => Ok(Primitive::Path(segments.clone())),
            _ => Err(Error::UnhandledPrimitive),
        }
    }
}

#[cfg(test)]
mod primitive_tests {
    use fabc_lexer::tokens::Token;

    use crate::{ast::expr::primitive::Primitive, Parsable, Parser};

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
    }
}
