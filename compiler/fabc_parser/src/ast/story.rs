use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{
    ast::{stmt::module::ModuleStmt, story::metadata::Metadata},
    Parsable,
};

pub mod metadata;

#[derive(Debug, PartialEq)]
pub struct Story {
    pub metadata: Option<Metadata>,
    pub modules: Vec<ModuleStmt>,
}

impl Parsable for Story {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        let mut modules = Vec::new();
        while parser.peek() == &Token::Keyword(KeywordKind::Module) {
            let module = ModuleStmt::parse(parser)?;
            modules.push(module);
        }

        let metadata = if parser.peek() == &Token::Keyword(KeywordKind::Story) {
            let metadata = Metadata::parse(parser)?;
            Some(metadata)
        } else {
            None
        };

        Ok(Story { metadata, modules })
    }
}

#[cfg(test)]
mod story_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            stmt::module::ModuleStmt,
            story::{metadata::Metadata, Story},
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_story_with_metadata_and_modules() {
        let source = r#"
            module "path/to/module1" as mod1
            module "path/to/module2"

            Story {
                description: "This is a test story."
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source code");

        let mut parser = Parser::new(tokens);
        let story = Story::parse(&mut parser).expect("Failed to parse story");

        let expected = Story {
            metadata: Some({
                let mut map = HashMap::new();
                map.insert(
                    "description".to_string(),
                    Expr::Primary(Primary::Literal(Literal::String(
                        "This is a test story.".to_string(),
                    ))),
                );
                Metadata { map }
            }),
            modules: vec![
                ModuleStmt {
                    path: "path/to/module1".to_string(),
                    alias: Some("mod1".to_string()),
                },
                ModuleStmt {
                    alias: None,
                    path: "path/to/module2".to_string(),
                },
            ],
        };

        assert_eq!(story, expected);
    }
}
