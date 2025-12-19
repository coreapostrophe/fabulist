use fabc_lexer::tokens::Token;

use crate::{ast::story::part::element::dialogue::quote::Quote, expect_token, Parsable};

pub mod quote;

#[derive(Debug, PartialEq)]
pub struct Dialogue {
    pub speaker: String,
    pub quotes: Vec<Quote>,
}

impl Parsable for Dialogue {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        let speaker = parser.enclosed(Token::LeftBracket, Token::RightBracket, |parser| {
            expect_token!(parser, Token::Identifier, "speaker identifier")
        })?;

        let mut quotes = Vec::new();
        while parser.peek() == &Token::Greater {
            let quote = Quote::parse(parser)?;
            quotes.push(quote);
        }

        Ok(Dialogue { speaker, quotes })
    }
}

#[cfg(test)]
mod dialogue_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            story::part::element::dialogue::{quote::Quote, Dialogue},
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_dialogue_element() {
        let source = r#"
            [narrator]
            > "Hello there!" { emotion: "happy", volume: 5 }
            > "How are you?" { emotion: "curious" }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(tokens);
        let dialogue = Dialogue::parse(&mut parser).expect("Failed to parse dialogue");

        let expected = Dialogue {
            speaker: "narrator".to_string(),
            quotes: vec![
                Quote {
                    text: "Hello there!".to_string(),
                    properties: Some({
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            "emotion".to_string(),
                            Expr::Primary(Primary::Literal(Literal::String("happy".to_string()))),
                        );
                        map.insert(
                            "volume".to_string(),
                            Expr::Primary(Primary::Literal(Literal::Number(5.0))),
                        );
                        map
                    }),
                },
                Quote {
                    text: "How are you?".to_string(),
                    properties: Some({
                        let mut map = HashMap::new();
                        map.insert(
                            "emotion".to_string(),
                            Expr::Primary(Primary::Literal(Literal::String("curious".to_string()))),
                        );
                        map
                    }),
                },
            ],
        };

        assert_eq!(dialogue, expected);
    }
}
