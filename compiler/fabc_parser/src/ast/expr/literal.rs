use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

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
            TokenKind::Keyword(KeywordKind::True) => Ok(Literal::Boolean(true)),
            TokenKind::Keyword(KeywordKind::False) => Ok(Literal::Boolean(false)),
            TokenKind::String(value) => Ok(Literal::String(value.to_string())),
            TokenKind::Number(value) => Ok(Literal::Number(*value)),
            TokenKind::Keyword(KeywordKind::None) => Ok(Literal::None),
            _ => Err(Error::UnhandledLiteral),
        }
    }
}

#[cfg(test)]
mod literal_tests {
    use fabc_lexer::Lexer;

    use crate::{ast::expr::literal::Literal, Parsable, Parser};

    #[test]
    fn parses_literals() {
        let source = "42";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");
        let mut parser = Parser::new(&tokens);
        assert_eq!(Literal::parse(&mut parser).unwrap(), Literal::Number(42.0));

        let source = "\"hello\"";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");
        let mut parser = Parser::new(&tokens);
        assert_eq!(
            Literal::parse(&mut parser).unwrap(),
            Literal::String("hello".to_string())
        );

        let source = "true";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");
        let mut parser = Parser::new(&tokens);
        assert_eq!(Literal::parse(&mut parser).unwrap(), Literal::Boolean(true));

        let source = "false";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");
        let mut parser = Parser::new(&tokens);
        assert_eq!(
            Literal::parse(&mut parser).unwrap(),
            Literal::Boolean(false)
        );

        let source = "none";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");
        let mut parser = Parser::new(&tokens);
        assert_eq!(Literal::parse(&mut parser).unwrap(), Literal::None);
    }
}
