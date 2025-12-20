use std::collections::HashMap;

use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::object::ObjectDecl, expr::Expr},
    expect_token, Parsable,
};

#[derive(Debug, PartialEq)]
pub struct Quote {
    pub text: String,
    pub properties: Option<HashMap<String, Expr>>,
}

impl Parsable for Quote {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(TokenKind::Greater)?;

        let text = expect_token!(parser, TokenKind::String, "quote text")?;

        let properties = if parser.peek() == &TokenKind::LeftBrace {
            Some(ObjectDecl::parse(parser)?.map)
        } else {
            None
        };

        Ok(Quote { text, properties })
    }
}

#[cfg(test)]
mod quote_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            story::part::element::dialogue::quote::Quote,
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_quote_without_properties() {
        let source = "> \"This is a quote.\"";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(&tokens);
        let quote = Quote::parse(&mut parser).expect("Failed to parse quote");

        let expected = Quote {
            text: "This is a quote.".to_string(),
            properties: None,
        };

        assert_eq!(quote, expected);
    }

    #[test]
    fn parses_quote_with_properties() {
        let source = "> \"This is a quote.\" { emotion: \"happy\", volume: 5 }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(&tokens);
        let quote = Quote::parse(&mut parser).expect("Failed to parse quote");

        let expected = Quote {
            text: "This is a quote.".to_string(),
            properties: Some({
                let mut map = HashMap::new();
                map.insert(
                    "emotion".to_string(),
                    Expr::Primary(Primary::Literal(Literal::String("happy".to_string()))),
                );
                map.insert(
                    "volume".to_string(),
                    Expr::Primary(Primary::Literal(Literal::Number(5.0))),
                );
                map
            }),
        };
        assert_eq!(quote, expected);
    }
}
