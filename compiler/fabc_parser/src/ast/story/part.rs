use fabc_lexer::tokens::TokenKind;

use crate::{ast::story::part::element::Element, expect_token, Parsable};

pub mod element;

#[derive(Debug, PartialEq)]
pub struct Part {
    pub id: String,
    pub elements: Vec<Element>,
}

impl Parsable for Part {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(TokenKind::Pound)?;

        let id = expect_token!(parser, TokenKind::Identifier, "identifier")?;

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

        Ok(Part { id, elements })
    }
}

#[cfg(test)]
mod part_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::story::part::{
            element::{narration::Narration, Element},
            Part,
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_part() {
        let source = r##"
            # intro
            * "This is a narration."
        "##;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");

        let mut parser = Parser::new(&tokens);
        let part = Part::parse(&mut parser).expect("Failed to parse part");

        let expected = Part {
            id: "intro".to_string(),
            elements: vec![Element::Narration(Narration {
                text: "This is a narration.".to_string(),
                properties: None,
            })],
        };

        assert_eq!(part, expected);
    }
}
