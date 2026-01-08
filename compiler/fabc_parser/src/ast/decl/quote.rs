use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::object::ObjectDecl, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct QuoteDecl {
    pub info: NodeInfo,
    pub text: String,
    pub properties: Option<ObjectDecl>,
}

impl Parsable for QuoteDecl {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();

        let text = expect_token!(parser, TokenKind::String, "quote text")?;
        let properties = if parser.peek() == &TokenKind::LeftBrace {
            Some(ObjectDecl::parse(parser)?)
        } else {
            None
        };

        let end_span = parser.end_span();

        Ok(QuoteDecl {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            text,
            properties,
        })
    }
}

#[cfg(test)]
mod quote_decl_tests {
    use std::collections::HashMap;

    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::{object::ObjectDecl, quote::QuoteDecl},
            expr::{literal::Literal, Expr, Primary},
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_quote_decl_without_properties() {
        let source = "\"This is a quote.\"";
        let tokens = Lexer::tokenize(source);
        let quote_decl = Parser::parse_ast::<QuoteDecl>(&tokens).expect("Failed to parse quote");

        let expected = QuoteDecl {
            info: NodeInfo {
                id: 0,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 18))),
            },
            text: "This is a quote.".to_string(),
            properties: None,
        };

        assert_eq!(quote_decl, expected);
    }

    #[test]
    fn parses_quote_decl_with_properties() {
        let source = "\"This is a quote with properties.\" { author: \"Alice\", length: 30 }";
        let tokens = Lexer::tokenize(source);
        let quote_decl = Parser::parse_ast::<QuoteDecl>(&tokens).expect("Failed to parse quote");

        let expected = QuoteDecl {
            info: NodeInfo {
                id: 3,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 66))),
            },
            text: "This is a quote with properties.".to_string(),
            properties: Some(ObjectDecl {
                info: NodeInfo {
                    id: 2,
                    span: Span::from((LineCol::new(1, 36), LineCol::new(1, 66))),
                },
                map: {
                    let mut map = HashMap::new();
                    map.insert(
                        "author".to_string(),
                        Expr::Primary {
                            info: NodeInfo {
                                id: 0,
                                span: Span::from((LineCol::new(1, 46), LineCol::new(1, 52))),
                            },
                            value: Primary::Literal(Literal::String("Alice".to_string())),
                        },
                    );
                    map.insert(
                        "length".to_string(),
                        Expr::Primary {
                            info: NodeInfo {
                                id: 1,
                                span: Span::from((LineCol::new(1, 63), LineCol::new(1, 64))),
                            },
                            value: Primary::Literal(Literal::Number(30.0)),
                        },
                    );
                    map
                },
            }),
        };
        assert_eq!(quote_decl, expected);
    }
}
