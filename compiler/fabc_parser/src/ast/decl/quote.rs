use fabc_lexer::tokens::TokenKind;

use crate::{ast::decl::object::ObjectDecl, expect_token, Parsable};

#[derive(Debug, PartialEq)]
pub struct QuoteDecl {
    pub id: usize,
    pub text: String,
    pub properties: Option<ObjectDecl>,
}

impl Parsable for QuoteDecl {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
        let text = expect_token!(parser, fabc_lexer::tokens::TokenKind::String, "quote text")?;

        let properties = if parser.peek() == &TokenKind::LeftBrace {
            Some(ObjectDecl::parse(parser)?)
        } else {
            None
        };

        Ok(QuoteDecl {
            id: parser.assign_id(),
            text,
            properties,
        })
    }
}

#[cfg(test)]
mod quote_decl_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::{object::ObjectDecl, quote::QuoteDecl},
            expr::{literal::Literal, Expr, Primary},
        },
        Parser,
    };

    #[test]
    fn parses_quote_decl_without_properties() {
        let source = "\"This is a quote.\"";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let quote_decl = Parser::parse::<QuoteDecl>(&tokens).expect("Failed to parse quote");

        let expected = QuoteDecl {
            id: 0,
            text: "This is a quote.".to_string(),
            properties: None,
        };

        assert_eq!(quote_decl, expected);
    }

    #[test]
    fn parses_quote_decl_with_properties() {
        let source = "\"This is a quote with properties.\" { author: \"Alice\", length: 30 }";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let quote_decl = Parser::parse::<QuoteDecl>(&tokens).expect("Failed to parse quote");

        let expected = QuoteDecl {
            id: 1,
            text: "This is a quote with properties.".to_string(),
            properties: Some(ObjectDecl {
                id: 0,
                map: {
                    let mut map = HashMap::new();
                    map.insert(
                        "author".to_string(),
                        Expr::Primary(Primary::Literal(Literal::String("Alice".to_string()))),
                    );
                    map.insert(
                        "length".to_string(),
                        Expr::Primary(Primary::Literal(Literal::Number(30.0))),
                    );
                    map
                },
            }),
        };
        assert_eq!(quote_decl, expected);
    }
}
