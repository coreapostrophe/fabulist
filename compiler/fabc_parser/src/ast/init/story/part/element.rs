use fabc_error::{kind::ErrorKind, Error};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::init::story::part::element::{
        dialogue::DialogueElement, narration::NarrationElement, selection::SelectionElement,
    },
    Parsable, Parser,
};

pub mod dialogue;
pub mod narration;
pub mod selection;

#[derive(Debug, PartialEq)]
pub enum Element {
    Narration(NarrationElement),
    Dialogue(DialogueElement),
    Selection(SelectionElement),
}

impl Element {
    pub const SYNC_DELIMITERS: &[TokenKind<'_>] = &[
        TokenKind::Minus,
        TokenKind::LeftBracket,
        TokenKind::Asterisk,
    ];
}

impl Parsable for Element {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        match parser.peek() {
            TokenKind::Minus => Ok(Element::Selection(SelectionElement::parse(parser)?)),
            TokenKind::LeftBracket => Ok(Element::Dialogue(DialogueElement::parse(parser)?)),
            TokenKind::Asterisk => Ok(Element::Narration(NarrationElement::parse(parser)?)),
            _ => Err(Error::new(
                ErrorKind::UnrecognizedElement {
                    element: parser.peek().to_string(),
                },
                parser.current_token(),
            )),
        }
    }
}
