use fabc_error::{kind::CompileErrorKind, Error};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::init::{module::ModuleInit, story::StoryInit},
    Parsable, Parser,
};

pub mod module;
pub mod story;

#[derive(Debug, PartialEq)]
pub enum Init {
    Story(StoryInit),
    Module(ModuleInit),
}

impl Init {
    pub const SYNC_DELIMITERS: &[TokenKind<'_>] = &[
        TokenKind::Keyword(KeywordKind::Story),
        TokenKind::Keyword(KeywordKind::Module),
    ];
}

impl Parsable for Init {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        match parser.peek() {
            TokenKind::Keyword(KeywordKind::Story) => Ok(Init::Story(StoryInit::parse(parser)?)),
            TokenKind::Keyword(KeywordKind::Module) => Ok(Init::Module(ModuleInit::parse(parser)?)),
            _ => Err(Error::new(
                CompileErrorKind::UnrecognizedInitiator {
                    initiator: parser.peek().to_string(),
                },
                parser.peek_token(),
            )),
        }
    }
}
