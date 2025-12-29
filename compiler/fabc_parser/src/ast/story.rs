use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{
        stmt::module::ModuleStmt,
        story::{metadata::Metadata, part::Part},
        Node, NodeId,
    },
    Parsable,
};

pub mod metadata;
pub mod part;

#[derive(Debug, PartialEq)]
pub struct Story {
    pub metadata: Option<Metadata>,
    pub modules: Option<Vec<ModuleStmt>>,
    pub parts: Vec<NodeId>,
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
            let part_id = parser.node_collection.add_node(Node::Part(part));
            parts.push(part_id);
        }

        Ok(Story {
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
            decl::quote::QuoteDecl,
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
        let story = Parser::collected_parse::<Story>(&tokens).expect("Failed to parse story");

        let expected_metadata = Metadata {
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
        };
        assert_eq!(story.ast.metadata, Some(expected_metadata));

        let expected_modules = vec![
            ModuleStmt {
                path: "path/to/module1".to_string(),
                alias: Some("mod1".to_string()),
            },
            ModuleStmt {
                path: "path/to/module2".to_string(),
                alias: None,
            },
        ];
        assert_eq!(story.ast.modules, Some(expected_modules));

        assert!(story.ast.parts.is_empty());
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
        let story = Parser::collected_parse::<Story>(&tokens).expect("Failed to parse story");

        let expected_metadata = Metadata {
            map: {
                let mut map = HashMap::new();
                map.insert(
                    "start".to_string(),
                    Expr::Primary(Primary::Literal(Literal::String("dialogue_1".to_string()))),
                );
                map
            },
        };
        assert_eq!(story.ast.metadata, Some(expected_metadata));

        let expected_modules = vec![ModuleStmt {
            path: "path/to/module".to_string(),
            alias: Some("dialogues".to_string()),
        }];
        assert_eq!(story.ast.modules, Some(expected_modules));

        let expected_parts = [Part {
            id: "dialogue_1".to_string(),
            elements: vec![
                Element::Narration(Narration {
                    text: "Welcome to the story!".to_string(),
                    properties: None,
                }),
                Element::Dialogue(Dialogue {
                    speaker: "traveller".to_string(),
                    quotes: vec![
                        QuoteDecl {
                            text: "Hello there!".to_string(),
                            properties: None,
                        },
                        QuoteDecl {
                            text: "Choose your path.".to_string(),
                            properties: None,
                        },
                    ],
                }),
                Element::Selection(Selection {
                    choices: vec![
                        QuoteDecl {
                            text: "Go left.".to_string(),
                            properties: Some({
                                let mut map = HashMap::new();
                                map.insert(
                                    "score".to_string(),
                                    Expr::Primary(Primary::Literal(Literal::Number(10.0))),
                                );
                                map
                            }),
                        },
                        QuoteDecl {
                            text: "Go right.".to_string(),
                            properties: Some({
                                let mut map = HashMap::new();
                                map.insert(
                                    "score".to_string(),
                                    Expr::Primary(Primary::Literal(Literal::Number(5.0))),
                                );
                                map
                            }),
                        },
                    ],
                }),
            ],
        }];

        let extrapolated_parts = story
            .node_collection
            .get_multi_node_values::<Part>(&story.ast.parts);

        assert_eq!(
            extrapolated_parts,
            expected_parts.iter().collect::<Vec<&Part>>()
        );
    }
}
