use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::quote::QuoteDecl, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct DialogueElement {
    pub info: NodeInfo,
    pub speaker: String,
    pub quotes: Vec<QuoteDecl>,
}

impl Parsable for DialogueElement {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        let speaker =
            parser.enclosed(TokenKind::LeftBracket, TokenKind::RightBracket, |parser| {
                expect_token!(parser, TokenKind::Identifier, "speaker identifier")
            })?;

        let mut quotes = Vec::new();
        while parser.r#match(&[TokenKind::Greater]) {
            let quote = QuoteDecl::parse(parser);
            match quote {
                Ok(quote) => quotes.push(quote),
                Err(err) => parser.errors.push(err),
            }
        }

        let end_span = parser.end_span();

        Ok(DialogueElement {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            speaker,
            quotes,
        })
    }
}

#[cfg(test)]
mod dialogue_tests {
    use std::collections::HashMap;

    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::{object::ObjectDecl, quote::QuoteDecl},
            expr::{literal::Literal, Expr, Primary},
            init::story::part::element::dialogue::DialogueElement,
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_dialogue_element() {
        let source = r#"
            [narrator]
            > "Hello there!" { emotion: "happy", volume: 5 }
            > "How are you?" { emotion: "curious" }
        "#;
        let tokens = Lexer::tokenize(source);
        let dialogue =
            Parser::parse_ast::<DialogueElement>(&tokens).expect("Failed to parse dialogue");

        let expected = DialogueElement {
            info: NodeInfo {
                id: 7,
                span: Span::from((LineCol::new(2, 13), LineCol::new(4, 51))),
            },
            speaker: "narrator".to_string(),
            quotes: vec![
                QuoteDecl {
                    info: NodeInfo {
                        id: 3,
                        span: Span::from((LineCol::new(3, 15), LineCol::new(3, 60))),
                    },
                    text: "Hello there!".to_string(),
                    properties: Some(ObjectDecl {
                        info: NodeInfo {
                            id: 2,
                            span: Span::from((LineCol::new(3, 30), LineCol::new(3, 60))),
                        },
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "emotion".to_string(),
                                Expr::Primary {
                                    info: NodeInfo {
                                        id: 0,
                                        span: Span::from((
                                            LineCol::new(3, 41),
                                            LineCol::new(3, 47),
                                        )),
                                    },
                                    value: Primary::Literal(Literal::String("happy".to_string())),
                                },
                            );
                            map.insert(
                                "volume".to_string(),
                                Expr::Primary {
                                    info: NodeInfo {
                                        id: 1,
                                        span: Span::from((
                                            LineCol::new(3, 58),
                                            LineCol::new(3, 58),
                                        )),
                                    },
                                    value: Primary::Literal(Literal::Number(5.0)),
                                },
                            );
                            map
                        },
                    }),
                },
                QuoteDecl {
                    info: NodeInfo {
                        id: 6,
                        span: Span::from((LineCol::new(4, 15), LineCol::new(4, 51))),
                    },
                    text: "How are you?".to_string(),
                    properties: Some(ObjectDecl {
                        info: NodeInfo {
                            id: 5,
                            span: Span::from((LineCol::new(4, 30), LineCol::new(4, 51))),
                        },
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "emotion".to_string(),
                                Expr::Primary {
                                    info: NodeInfo {
                                        id: 4,
                                        span: Span::from((
                                            LineCol::new(4, 41),
                                            LineCol::new(4, 49),
                                        )),
                                    },
                                    value: Primary::Literal(Literal::String("curious".to_string())),
                                },
                            );
                            map
                        },
                    }),
                },
            ],
        };

        assert_eq!(dialogue, expected);
    }
}
