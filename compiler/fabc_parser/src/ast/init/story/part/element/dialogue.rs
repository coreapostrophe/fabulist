use fabc_error::Error;
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::quote::QuoteDecl, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct DialogueElement {
    pub info: NodeInfo,
    pub speaker: String,
    pub quotes: Vec<QuoteDecl>,
}

impl Parsable for DialogueElement {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let speaker =
            parser.enclosed(TokenKind::LeftBracket, TokenKind::RightBracket, |parser| {
                expect_token!(parser, TokenKind::Identifier, "speaker identifier")
            })?;

        let mut quotes = Vec::new();
        while parser.peek() == &TokenKind::Greater {
            let quote = parser.prefixed(TokenKind::Greater, |parser| QuoteDecl::parse(parser))?;
            quotes.push(quote);
        }

        Ok(DialogueElement {
            info: NodeInfo {
                id: parser.assign_id(),
            },
            speaker,
            quotes,
        })
    }
}

#[cfg(test)]
mod dialogue_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::{object::ObjectDecl, quote::QuoteDecl},
            expr::{literal::Literal, Expr, Primary},
            init::story::part::element::dialogue::DialogueElement,
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_dialogue_element() {
        let source = r#"
            [narrator]
            > "Hello there!" { emotion: "happy", volume: 5 }
            > "How are you?" { emotion: "curious" }
        "#;
        let tokens = Lexer::tokenize(source);
        let dialogue =
            Parser::parse_ast::<DialogueElement>(&tokens).expect("Failed to parse dialogue");

        let expected = DialogueElement {
            info: NodeInfo { id: 7 },
            speaker: "narrator".to_string(),
            quotes: vec![
                QuoteDecl {
                    info: NodeInfo { id: 3 },
                    text: "Hello there!".to_string(),
                    properties: Some(ObjectDecl {
                        info: NodeInfo { id: 2 },
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "emotion".to_string(),
                                Expr::Primary {
                                    info: NodeInfo { id: 0 },
                                    value: Primary::Literal(Literal::String("happy".to_string())),
                                },
                            );
                            map.insert(
                                "volume".to_string(),
                                Expr::Primary {
                                    info: NodeInfo { id: 1 },
                                    value: Primary::Literal(Literal::Number(5.0)),
                                },
                            );
                            map
                        },
                    }),
                },
                QuoteDecl {
                    info: NodeInfo { id: 6 },
                    text: "How are you?".to_string(),
                    properties: Some(ObjectDecl {
                        info: NodeInfo { id: 5 },
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "emotion".to_string(),
                                Expr::Primary {
                                    info: NodeInfo { id: 4 },
                                    value: Primary::Literal(Literal::String("curious".to_string())),
                                },
                            );
                            map
                        },
                    }),
                },
            ],
        };

        assert_eq!(dialogue, expected);
    }
}
