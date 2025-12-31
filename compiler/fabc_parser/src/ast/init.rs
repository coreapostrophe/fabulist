use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::init::{module::ModuleInit, story::StoryInit},
    error::Error,
    Parsable, Parser,
};

pub mod module;
pub mod story;

#[derive(Debug, PartialEq)]
pub enum Init {
    Story(StoryInit),
    Module(ModuleInit),
}

impl Parsable for Init {
    fn parse<'src, 'tok>(parser: &mut Parser<'src, 'tok>) -> Result<Self, Error> {
        match parser.peek() {
            TokenKind::Keyword(KeywordKind::Story) => Ok(Init::Story(StoryInit::parse(parser)?)),
            TokenKind::Keyword(KeywordKind::Module) => Ok(Init::Module(ModuleInit::parse(parser)?)),
            _ => Err(Error::UnhandledInitiator),
        }
    }
}
