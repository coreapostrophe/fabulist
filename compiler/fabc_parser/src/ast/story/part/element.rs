use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::story::part::element::{dialogue::Dialogue, narration::Narration, selection::Selection},
    error::Error,
    Parsable,
};

pub mod dialogue;
pub mod narration;
pub mod selection;

#[derive(Debug, PartialEq)]
pub enum Element {
    Narration(Narration),
    Dialogue(Dialogue),
    Selection(Selection),
}

impl Parsable for Element {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        match parser.peek() {
            TokenKind::Minus => Ok(Element::Selection(Selection::parse(parser)?)),
            TokenKind::LeftBracket => Ok(Element::Dialogue(Dialogue::parse(parser)?)),
            TokenKind::Asterisk => Ok(Element::Narration(Narration::parse(parser)?)),
            _ => Err(Error::UnhandledElement),
        }
    }
}
