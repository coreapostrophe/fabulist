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
    Narration { id: usize, value: Narration },
    Dialogue { id: usize, value: Dialogue },
    Selection { id: usize, value: Selection },
}

impl Parsable for Element {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
        match parser.peek() {
            TokenKind::Minus => {
                let value = Selection::parse(parser)?;
                Ok(Element::Selection {
                    id: parser.assign_id(),
                    value,
                })
            }
            TokenKind::LeftBracket => {
                let value = Dialogue::parse(parser)?;
                Ok(Element::Dialogue {
                    id: parser.assign_id(),
                    value,
                })
            }
            TokenKind::Asterisk => {
                let value = Narration::parse(parser)?;
                Ok(Element::Narration {
                    id: parser.assign_id(),
                    value,
                })
            }
            _ => Err(Error::UnhandledElement),
        }
    }
}
