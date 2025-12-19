use fabc_lexer::tokens::Token;

use crate::{ast::story::part::element::Element, error::Error, Parsable};

pub mod element;

#[derive(Debug, PartialEq)]
pub struct Part {
    id: String,
    elements: Vec<Element>,
}

impl Parsable for Part {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(Token::Pound)?;

        let id = if let Token::Identifier(id) = parser.advance() {
            id.clone()
        } else {
            return Err(Error::ExpectedFound {
                expected: "identifier".to_string(),
                found: parser.previous().to_string(),
            });
        };

        let mut elements = Vec::new();

        while [Token::Asterisk].contains(parser.peek()) {
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
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(tokens);
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
