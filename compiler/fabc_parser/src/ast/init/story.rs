use fabc_error::{Error, Span};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{
        init::{
            story::{metadata::Metadata, part::Part},
            Init,
        },
        NodeInfo,
    },
    Parsable, Parser,
};

pub mod metadata;
pub mod part;

#[derive(Debug, PartialEq)]
pub struct StoryInit {
    pub info: NodeInfo,
    pub metadata: Option<Metadata>,
    pub parts: Vec<Part>,
}

impl Parsable for StoryInit {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        let metadata = if parser.peek() == &TokenKind::Keyword(KeywordKind::Story) {
            let metadata = Metadata::parse(parser)?;
            Some(metadata)
        } else {
            None
        };

        let parts = parser.invariant_parse(Part::SYNC_DELIMITERS, Init::SYNC_DELIMITERS, false);

        let end_span = parser.end_span();

        Ok(StoryInit {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            metadata,
            parts,
        })
    }
}

#[cfg(test)]
mod story_tests {
    use std::collections::HashMap;

    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::{object::ObjectDecl, quote::QuoteDecl},
            expr::{literal::Literal, Expr, Primary},
            init::story::{
                metadata::Metadata,
                part::{
                    element::{
                        dialogue::DialogueElement, narration::NarrationElement,
                        selection::SelectionElement, Element,
                    },
                    Part,
                },
                StoryInit,
            },
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_story_with_metadata_and_modules() {
        let source = r#"
            Story {
                description: "This is a test story."
            }
        "#;
        let tokens = Lexer::tokenize(source);
        let story = Parser::parse_ast::<StoryInit>(&tokens).expect("Failed to parse story");

        let expected = StoryInit {
            info: NodeInfo {
                id: 3,
                span: Span::from((LineCol::new(2, 13), LineCol::new(4, 13))),
            },
            metadata: Some(Metadata {
                info: NodeInfo {
                    id: 2,
                    span: Span::from((LineCol::new(2, 13), LineCol::new(4, 13))),
                },
                object: ObjectDecl {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(2, 19), LineCol::new(4, 13))),
                    },
                    map: {
                        let mut map = HashMap::new();
                        map.insert(
                            "description".to_string(),
                            Expr::Primary {
                                info: NodeInfo {
                                    id: 0,
                                    span: Span::from((LineCol::new(3, 30), LineCol::new(3, 52))),
                                },
                                value: Primary::Literal(Literal::String(
                                    "This is a test story.".to_string(),
                                )),
                            },
                        );
                        map
                    },
                },
            }),
            parts: vec![],
        };
        assert_eq!(story, expected);
    }

    #[test]
    fn parses_basic_story() {
        let source = r#"
            Story {
                start: "dialogue_1"
            }

            # dialogue_1
            * "Welcome to the story!"
            [traveller]
            > "Hello there!"
            > "Choose your path."
                - "Go left." { score: 10 }
                - "Go right." { score: 5 }
        "#;
        let tokens = Lexer::tokenize(source);
        let story = Parser::parse_ast::<StoryInit>(&tokens).expect("Failed to parse story");

        let expected = StoryInit {
            info: NodeInfo {
                id: 16,
                span: Span::from((LineCol::new(2, 13), LineCol::new(12, 42))),
            },
            metadata: Some(Metadata {
                info: NodeInfo {
                    id: 2,
                    span: Span::from((LineCol::new(2, 13), LineCol::new(4, 13))),
                },
                object: ObjectDecl {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(2, 19), LineCol::new(4, 13))),
                    },
                    map: {
                        let mut map = HashMap::new();
                        map.insert(
                            "start".to_string(),
                            Expr::Primary {
                                info: NodeInfo {
                                    id: 0,
                                    span: Span::from((LineCol::new(3, 24), LineCol::new(3, 35))),
                                },
                                value: Primary::Literal(Literal::String("dialogue_1".to_string())),
                            },
                        );
                        map
                    },
                },
            }),
            parts: vec![Part {
                info: NodeInfo {
                    id: 15,
                    span: Span::from((LineCol::new(6, 13), LineCol::new(12, 42))),
                },
                ident: "dialogue_1".to_string(),
                elements: vec![
                    Element::Narration(NarrationElement {
                        info: NodeInfo {
                            id: 4,
                            span: Span::from((LineCol::new(7, 13), LineCol::new(7, 37))),
                        },
                        quote: QuoteDecl {
                            info: NodeInfo {
                                id: 3,
                                span: Span::from((LineCol::new(7, 15), LineCol::new(7, 37))),
                            },
                            text: "Welcome to the story!".to_string(),
                            properties: None,
                        },
                    }),
                    Element::Dialogue(DialogueElement {
                        info: NodeInfo {
                            id: 7,
                            span: Span::from((LineCol::new(8, 13), LineCol::new(10, 33))),
                        },
                        speaker: "traveller".to_string(),
                        quotes: vec![
                            QuoteDecl {
                                info: NodeInfo {
                                    id: 5,
                                    span: Span::from((LineCol::new(9, 15), LineCol::new(9, 28))),
                                },
                                text: "Hello there!".to_string(),
                                properties: None,
                            },
                            QuoteDecl {
                                info: NodeInfo {
                                    id: 6,
                                    span: Span::from((LineCol::new(10, 15), LineCol::new(10, 33))),
                                },
                                text: "Choose your path.".to_string(),
                                properties: None,
                            },
                        ],
                    }),
                    Element::Selection(SelectionElement {
                        info: NodeInfo {
                            id: 14,
                            span: Span::from((LineCol::new(11, 17), LineCol::new(12, 42))),
                        },
                        choices: vec![
                            QuoteDecl {
                                info: NodeInfo {
                                    id: 10,
                                    span: Span::from((LineCol::new(11, 19), LineCol::new(11, 42))),
                                },
                                text: "Go left.".to_string(),
                                properties: Some(ObjectDecl {
                                    info: NodeInfo {
                                        id: 9,
                                        span: Span::from((
                                            LineCol::new(11, 30),
                                            LineCol::new(11, 42),
                                        )),
                                    },
                                    map: {
                                        let mut map = HashMap::new();
                                        map.insert(
                                            "score".to_string(),
                                            Expr::Primary {
                                                info: NodeInfo {
                                                    id: 8,
                                                    span: Span::from((
                                                        LineCol::new(11, 39),
                                                        LineCol::new(11, 40),
                                                    )),
                                                },
                                                value: Primary::Literal(Literal::Number(10.0)),
                                            },
                                        );
                                        map
                                    },
                                }),
                            },
                            QuoteDecl {
                                info: NodeInfo {
                                    id: 13,
                                    span: Span::from((LineCol::new(12, 19), LineCol::new(12, 42))),
                                },
                                text: "Go right.".to_string(),
                                properties: Some(ObjectDecl {
                                    info: NodeInfo {
                                        id: 12,
                                        span: Span::from((
                                            LineCol::new(12, 31),
                                            LineCol::new(12, 42),
                                        )),
                                    },
                                    map: {
                                        let mut map = HashMap::new();
                                        map.insert(
                                            "score".to_string(),
                                            Expr::Primary {
                                                info: NodeInfo {
                                                    id: 11,
                                                    span: Span::from((
                                                        LineCol::new(12, 40),
                                                        LineCol::new(12, 40),
                                                    )),
                                                },
                                                value: Primary::Literal(Literal::Number(5.0)),
                                            },
                                        );
                                        map
                                    },
                                }),
                            },
                        ],
                    }),
                ],
            }],
        };
        assert_eq!(story, expected);
    }
}
