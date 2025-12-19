use std::collections::HashMap;

use fabc_lexer::tokens::Token;

use crate::{ast::expr::Expr, Parsable};

#[derive(Debug, PartialEq)]
pub struct Narration {
    pub text: String,
    pub properties: Option<HashMap<String, Expr>>,
}

impl Parsable for Narration {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(Token::Asterisk)?;

        let text = if let Token::String(text) = parser.advance() {
            text.clone()
        } else {
            return Err(crate::error::Error::ExpectedFound {
                expected: "string literal".to_string(),
                found: parser.previous().to_string(),
            });
        };

        let properties = if parser.peek() == &Token::LeftBrace {
            let map_vec = parser.punctuated(
                Token::LeftBrace,
                Token::RightBrace,
                Token::Comma,
                |parser| {
                    let key = if let Token::Identifier(key) = parser.advance() {
                        key.clone()
                    } else {
                        return Err(crate::error::Error::ExpectedFound {
                            expected: "identifier".to_string(),
                            found: parser.previous().to_string(),
                        });
                    };

                    parser.consume(Token::Colon)?;

                    let value = Expr::parse(parser)?;

                    Ok((key, value))
                },
            )?;

            let mut map = HashMap::new();
            for (key, value) in map_vec {
                map.insert(key, value);
            }

            Some(map)
        } else {
            None
        };

        Ok(Narration { text, properties })
    }
}

#[cfg(test)]
mod narration_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{primitive::Primitive, Expr, Primary},
            story::part::element::narration::Narration,
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_narration_without_properties() {
        let source = "* \"This is a narration.\"";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(tokens);
        let narration = Narration::parse(&mut parser).expect("Failed to parse narration");

        let expected = Narration {
            text: "This is a narration.".to_string(),
            properties: None,
        };

        assert_eq!(narration, expected);
    }

    #[test]
    fn parses_narration_with_properties() {
        let source = "* \"This is a narration.\" { mood: happy, volume: loud }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(tokens);
        let narration = Narration::parse(&mut parser).expect("Failed to parse narration");

        let expected = Narration {
            text: "This is a narration.".to_string(),
            properties: Some({
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "mood".to_string(),
                    Expr::Primary(Primary::Primitive(Primitive::Identifier(
                        "happy".to_string(),
                    ))),
                );
                map.insert(
                    "volume".to_string(),
                    Expr::Primary(Primary::Primitive(Primitive::Identifier(
                        "loud".to_string(),
                    ))),
                );
                map
            }),
        };

        assert_eq!(narration, expected);
    }
}
