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
    fn parse<'src, 'tok>(parser: &mut crate::Parser<'src, 'tok>) -> Result<Self, Error> {
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

    use crate::{ast::expr::literal::Literal, Parser};

    #[test]
    fn parses_literals() {
        let source = "42";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        assert_eq!(literal, Literal::Number(42.0));

        let source = "\"hello\"";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        assert_eq!(literal, Literal::String("hello".to_string()));

        let source = "true";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        assert_eq!(literal, Literal::Boolean(true));

        let source = "false";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        assert_eq!(literal, Literal::Boolean(false));

        let source = "none";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        assert_eq!(literal, Literal::None);
    }
}
