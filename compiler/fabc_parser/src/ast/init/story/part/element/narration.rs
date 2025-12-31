use fabc_lexer::tokens::TokenKind;

use crate::{ast::decl::quote::QuoteDecl, Parsable};

#[derive(Debug, PartialEq)]
pub struct Narration {
    pub id: usize,
    pub quote: QuoteDecl,
}

impl Parsable for Narration {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
        parser.consume(TokenKind::Asterisk)?;

        let quote = QuoteDecl::parse(parser)?;

        Ok(Narration {
            id: parser.assign_id(),
            quote,
        })
    }
}

#[cfg(test)]
mod narration_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::{object::ObjectDecl, quote::QuoteDecl},
            expr::{primitive::Primitive, Expr, Primary},
            init::story::part::element::narration::Narration,
        },
        Parser,
    };

    #[test]
    fn parses_narration_without_properties() {
        let source = "* \"This is a narration.\"";
        let tokens = Lexer::tokenize(source);
        let narration = Parser::parse_ast::<Narration>(&tokens).expect("Failed to parse narration");

        let expected = Narration {
            id: 1,
            quote: QuoteDecl {
                id: 0,
                text: "This is a narration.".to_string(),
                properties: None,
            },
        };

        assert_eq!(narration, expected);
    }

    #[test]
    fn parses_narration_with_properties() {
        let source = "* \"This is a narration.\" { mood: happy, volume: loud }";
        let tokens = Lexer::tokenize(source);
        let narration = Parser::parse_ast::<Narration>(&tokens).expect("Failed to parse narration");

        let expected = Narration {
            id: 6,
            quote: QuoteDecl {
                id: 5,
                text: "This is a narration.".to_string(),
                properties: Some(ObjectDecl {
                    id: 4,
                    map: {
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            "mood".to_string(),
                            Expr::Primary {
                                id: 1,
                                value: Primary::Primitive(Primitive::Identifier {
                                    id: 0,
                                    name: "happy".to_string(),
                                }),
                            },
                        );
                        map.insert(
                            "volume".to_string(),
                            Expr::Primary {
                                id: 3,
                                value: Primary::Primitive(Primitive::Identifier {
                                    id: 2,
                                    name: "loud".to_string(),
                                }),
                            },
                        );
                        map
                    },
                }),
            },
        };

        assert_eq!(narration, expected);
    }
}
