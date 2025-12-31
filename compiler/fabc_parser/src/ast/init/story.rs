use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::init::story::{metadata::Metadata, part::Part},
    error::Error,
    Parsable, Parser,
};

pub mod metadata;
pub mod part;

#[derive(Debug, PartialEq)]
pub struct StoryInit {
    pub id: usize,
    pub metadata: Option<Metadata>,
    pub parts: Vec<Part>,
}

impl Parsable for StoryInit {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let metadata = if parser.peek() == &TokenKind::Keyword(KeywordKind::Story) {
            let metadata = Metadata::parse(parser)?;
            Some(metadata)
        } else {
            None
        };

        let mut parts = Vec::new();
        while parser.peek() == &TokenKind::Pound {
            let part = Part::parse(parser)?;
            parts.push(part);
        }

        Ok(StoryInit {
            id: parser.assign_id(),
            metadata,
            parts,
        })
    }
}

#[cfg(test)]
mod story_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::{object::ObjectDecl, quote::QuoteDecl},
            expr::{literal::Literal, Expr, Primary},
            init::story::{
                metadata::Metadata,
                part::{
                    element::{
                        dialogue::Dialogue, narration::Narration, selection::Selection, Element,
                    },
                    Part,
                },
                StoryInit,
            },
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
            id: 3,
            metadata: Some(Metadata {
                id: 2,
                object: ObjectDecl {
                    id: 1,
                    map: {
                        let mut map = HashMap::new();
                        map.insert(
                            "description".to_string(),
                            Expr::Primary {
                                id: 0,
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
            id: 16,
            metadata: Some(Metadata {
                id: 2,
                object: ObjectDecl {
                    id: 1,
                    map: {
                        let mut map = HashMap::new();
                        map.insert(
                            "start".to_string(),
                            Expr::Primary {
                                id: 0,
                                value: Primary::Literal(Literal::String("dialogue_1".to_string())),
                            },
                        );
                        map
                    },
                },
            }),
            parts: vec![Part {
                id: 15,
                ident: "dialogue_1".to_string(),
                elements: vec![
                    Element::Narration(Narration {
                        id: 4,
                        quote: QuoteDecl {
                            id: 3,
                            text: "Welcome to the story!".to_string(),
                            properties: None,
                        },
                    }),
                    Element::Dialogue(Dialogue {
                        id: 7,
                        speaker: "traveller".to_string(),
                        quotes: vec![
                            QuoteDecl {
                                id: 5,
                                text: "Hello there!".to_string(),
                                properties: None,
                            },
                            QuoteDecl {
                                id: 6,
                                text: "Choose your path.".to_string(),
                                properties: None,
                            },
                        ],
                    }),
                    Element::Selection(Selection {
                        id: 14,
                        choices: vec![
                            QuoteDecl {
                                id: 10,
                                text: "Go left.".to_string(),
                                properties: Some(ObjectDecl {
                                    id: 9,
                                    map: {
                                        let mut map = HashMap::new();
                                        map.insert(
                                            "score".to_string(),
                                            Expr::Primary {
                                                id: 8,
                                                value: Primary::Literal(Literal::Number(10.0)),
                                            },
                                        );
                                        map
                                    },
                                }),
                            },
                            QuoteDecl {
                                id: 13,
                                text: "Go right.".to_string(),
                                properties: Some(ObjectDecl {
                                    id: 12,
                                    map: {
                                        let mut map = HashMap::new();
                                        map.insert(
                                            "score".to_string(),
                                            Expr::Primary {
                                                id: 11,
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
