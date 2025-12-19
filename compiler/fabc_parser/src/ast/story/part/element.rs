use fabc_lexer::tokens::Token;

use crate::{ast::story::part::element::narration::Narration, error::Error, Parsable};

pub mod narration;

#[derive(Debug, PartialEq)]
pub enum Element {
    Narration(Narration),
}

impl Parsable for Element {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        if parser.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }
        println!("Parsing Part, current token: {:?}", parser.peek());

        match parser.peek() {
            Token::Asterisk => Ok(Element::Narration(Narration::parse(parser)?)),
            _ => Err(Error::UnhandledElement),
        }
    }
}

#[cfg(test)]
mod element_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::story::part::element::{narration::Narration, Element},
        Parsable, Parser,
    };

    #[test]
    fn parses_element() {
        let source = "* \"This is a narration.\"";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(tokens);
        let element = Element::parse(&mut parser).expect("Failed to parse element");

        let expected = Element::Narration(Narration {
            text: "This is a narration.".to_string(),
            properties: None,
        });

        assert_eq!(element, expected);
    }
}
