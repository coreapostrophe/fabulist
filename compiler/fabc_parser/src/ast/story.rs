use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{ast::story::metadata::Metadata, Parsable};

pub mod metadata;

#[derive(Debug, PartialEq)]
pub struct Story {
    pub metadata: Option<Metadata>,
}

impl Parsable for Story {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        let metadata = if parser.peek() == &Token::Keyword(KeywordKind::Story) {
            let metadata = Metadata::parse(parser)?;
            Some(metadata)
        } else {
            None
        };

        Ok(Story { metadata })
    }
}
