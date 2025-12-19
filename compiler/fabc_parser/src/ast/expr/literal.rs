use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{error::Error, Parsable};

#[derive(Debug, PartialEq)]
pub enum Literal {
    Boolean(bool),
    String(String),
    Number(f64),
    None,
}

impl Parsable for Literal {
    fn parse(parser: &mut crate::Parser) -> Result<Self, Error> {
        match parser.advance() {
            Token::Keyword(KeywordKind::True) => Ok(Literal::Boolean(true)),
            Token::Keyword(KeywordKind::False) => Ok(Literal::Boolean(false)),
            Token::String(value) => Ok(Literal::String(value.to_string())),
            Token::Number(value) => Ok(Literal::Number(*value)),
            Token::Keyword(KeywordKind::None) => Ok(Literal::None),
            _ => Err(Error::UnhandledLiteral),
        }
    }
}

#[cfg(test)]
mod literal_tests {
    use fabc_lexer::{keywords::KeywordKind, tokens::Token};

    use crate::{ast::expr::literal::Literal, Parsable, Parser};

    #[test]
    fn parses_literals() {
        let tokens = vec![Token::Number(42.0)];
        let mut parser = Parser::new(&tokens);
        assert_eq!(Literal::parse(&mut parser).unwrap(), Literal::Number(42.0));

        let tokens = vec![Token::String("hello")];
        let mut parser = Parser::new(&tokens);
        assert_eq!(
            Literal::parse(&mut parser).unwrap(),
            Literal::String("hello".to_string())
        );

        let tokens = vec![Token::Keyword(KeywordKind::True)];
        let mut parser = Parser::new(&tokens);
        assert_eq!(Literal::parse(&mut parser).unwrap(), Literal::Boolean(true));

        let tokens = vec![Token::Keyword(KeywordKind::False)];
        let mut parser = Parser::new(&tokens);
        assert_eq!(
            Literal::parse(&mut parser).unwrap(),
            Literal::Boolean(false)
        );

        let tokens = vec![Token::Keyword(KeywordKind::None)];
        let mut parser = Parser::new(&tokens);
        assert_eq!(Literal::parse(&mut parser).unwrap(), Literal::None);
    }
}
