use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{
        stmt::module::ModuleStmt,
        story::{metadata::Metadata, part::Part},
    },
    Parsable,
};

pub mod metadata;
pub mod part;

#[derive(Debug, PartialEq)]
pub struct Story {
    pub id: usize,
    pub metadata: Option<Metadata>,
    pub modules: Option<Vec<ModuleStmt>>,
    pub parts: Vec<Part>,
}

impl Parsable for Story {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
        let modules = if parser.peek() == &TokenKind::Keyword(KeywordKind::Module) {
            let mut mods = Vec::new();
            while parser.peek() == &TokenKind::Keyword(KeywordKind::Module) {
                let module = ModuleStmt::parse(parser)?;
                mods.push(module);
            }
            Some(mods)
        } else {
            None
        };

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

        Ok(Story {
            id: parser.assign_id(),
            metadata,
            modules,
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
            stmt::module::ModuleStmt,
            story::{
                metadata::Metadata,
                part::{
                    element::{
                        dialogue::Dialogue, narration::Narration, selection::Selection, Element,
                    },
                    Part,
                },
                Story,
            },
        },
        Parser,
    };

    #[test]
    fn parses_story_with_metadata_and_modules() {
        let source = r#"
            module "path/to/module1" as mod1;
            module "path/to/module2";

            Story {
                description: "This is a test story."
            }
        "#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let story = Parser::parse::<Story>(&tokens).expect("Failed to parse story");

        let expected = Story {
            id: 4,
            modules: Some(vec![
                ModuleStmt {
                    id: 0,
                    path: "path/to/module1".to_string(),
                    alias: Some("mod1".to_string()),
                },
                ModuleStmt {
                    id: 1,
                    alias: None,
                    path: "path/to/module2".to_string(),
                },
            ]),
            metadata: Some(Metadata {
                id: 3,
                object: ObjectDecl {
                    id: 2,
                    map: {
                        let mut map = HashMap::new();
                        map.insert(
                            "description".to_string(),
                            Expr::Primary(Primary::Literal(Literal::String(
                                "This is a test story.".to_string(),
                            ))),
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
            module "path/to/module" as dialogues;

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
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let story = Parser::parse::<Story>(&tokens).expect("Failed to parse story");

        let expected = Story {
            id: 17,
            modules: Some(vec![ModuleStmt {
                id: 0,
                path: "path/to/module".to_string(),
                alias: Some("dialogues".to_string()),
            }]),
            metadata: Some(Metadata {
                id: 2,
                object: ObjectDecl {
                    id: 1,
                    map: {
                        let mut map = HashMap::new();
                        map.insert(
                            "start".to_string(),
                            Expr::Primary(Primary::Literal(Literal::String(
                                "dialogue_1".to_string(),
                            ))),
                        );
                        map
                    },
                },
            }),
            parts: vec![Part {
                id: 16,
                ident: "dialogue_1".to_string(),
                elements: vec![
                    Element::Narration {
                        id: 5,
                        value: Narration {
                            id: 4,
                            quote: QuoteDecl {
                                id: 3,
                                text: "Welcome to the story!".to_string(),
                                properties: None,
                            },
                        },
                    },
                    Element::Dialogue {
                        id: 9,
                        value: Dialogue {
                            id: 8,
                            speaker: "traveller".to_string(),
                            quotes: vec![
                                QuoteDecl {
                                    id: 6,
                                    text: "Hello there!".to_string(),
                                    properties: None,
                                },
                                QuoteDecl {
                                    id: 7,
                                    text: "Choose your path.".to_string(),
                                    properties: None,
                                },
                            ],
                        },
                    },
                    Element::Selection {
                        id: 15,
                        value: Selection {
                            id: 14,
                            choices: vec![
                                QuoteDecl {
                                    id: 11,
                                    text: "Go left.".to_string(),
                                    properties: Some(ObjectDecl {
                                        id: 10,
                                        map: {
                                            let mut map = HashMap::new();
                                            map.insert(
                                                "score".to_string(),
                                                Expr::Primary(Primary::Literal(Literal::Number(
                                                    10.0,
                                                ))),
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
                                                Expr::Primary(Primary::Literal(Literal::Number(
                                                    5.0,
                                                ))),
                                            );
                                            map
                                        },
                                    }),
                                },
                            ],
                        },
                    },
                ],
            }],
        };
        assert_eq!(story, expected);
    }
}
