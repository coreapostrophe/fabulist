use fabc_lexer::tokens::Token;

use crate::{ast::story::part::element::narration::Narration, error::Error, Parsable};

pub mod dialogue;
pub mod narration;

#[derive(Debug, PartialEq)]
pub enum Element {
    Narration(Narration),
}

impl Parsable for Element {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        match parser.peek() {
            Token::Asterisk => Ok(Element::Narration(Narration::parse(parser)?)),
            _ => Err(Error::UnhandledElement),
        }
    }
}
