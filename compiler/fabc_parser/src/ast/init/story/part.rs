use fabc_error::Error;
use fabc_lexer::tokens::TokenKind;

use crate::{ast::init::story::part::element::Element, expect_token, Parsable, Parser};

pub mod element;

#[derive(Debug, PartialEq)]
pub struct Part {
    pub id: usize,
    pub ident: String,
    pub elements: Vec<Element>,
}

impl Parsable for Part {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        parser.consume(TokenKind::Pound)?;

        let ident = expect_token!(parser, TokenKind::Identifier, "identifier")?;

        let mut elements = Vec::new();

        while [
            TokenKind::Asterisk,
            TokenKind::LeftBracket,
            TokenKind::Minus,
        ]
        .contains(parser.peek())
        {
            let element = Element::parse(parser)?;
            elements.push(element);
        }

        Ok(Part {
            id: parser.assign_id(),
            ident,
            elements,
        })
    }
}

#[cfg(test)]
mod part_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::quote::QuoteDecl,
            init::story::part::{
                element::{narration::Narration, Element},
                Part,
            },
        },
        Parser,
    };

    #[test]
    fn parses_part() {
        let source = r##"
            # intro
            * "This is a narration."
        "##;
        let tokens = Lexer::tokenize(source);
        let part = Parser::parse_ast::<Part>(&tokens).expect("Failed to parse part");

        let expected = Part {
            id: 2,
            ident: "intro".to_string(),
            elements: vec![Element::Narration(Narration {
                id: 1,
                quote: QuoteDecl {
                    id: 0,
                    text: "This is a narration.".to_string(),
                    properties: None,
                },
            })],
        };

        assert_eq!(part, expected);
    }
}
