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
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::init::story::StoryInit, Parser};

    #[test]
    fn parses_story_with_metadata_and_modules() {
        let story = Parser::parse_ast_str::<StoryInit>(
            r#"
            Story {
                description: "This is a test story."
            }
        "#,
        )
        .expect("Failed to parse story");

        assert_debug_snapshot!(story);
    }

    #[test]
    fn parses_basic_story() {
        let story = Parser::parse_ast_str::<StoryInit>(
            r#"
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
        "#,
        )
        .expect("Failed to parse story");

        assert_debug_snapshot!(story);
    }
}
