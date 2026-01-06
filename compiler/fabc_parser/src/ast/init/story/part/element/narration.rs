use fabc_error::Error;
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::quote::QuoteDecl, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct NarrationElement {
    pub info: NodeInfo,
    pub quote: QuoteDecl,
}

impl Parsable for NarrationElement {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        parser.consume(TokenKind::Asterisk)?;

        let quote = QuoteDecl::parse(parser)?;

        Ok(NarrationElement {
            info: NodeInfo {
                id: parser.assign_id(),
            },
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
            init::story::part::element::narration::NarrationElement,
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_narration_without_properties() {
        let source = "* \"This is a narration.\"";
        let tokens = Lexer::tokenize(source);
        let narration =
            Parser::parse_ast::<NarrationElement>(&tokens).expect("Failed to parse narration");

        let expected = NarrationElement {
            info: NodeInfo { id: 1 },
            quote: QuoteDecl {
                info: NodeInfo { id: 0 },
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
        let narration =
            Parser::parse_ast::<NarrationElement>(&tokens).expect("Failed to parse narration");

        let expected = NarrationElement {
            info: NodeInfo { id: 6 },
            quote: QuoteDecl {
                info: NodeInfo { id: 5 },
                text: "This is a narration.".to_string(),
                properties: Some(ObjectDecl {
                    info: NodeInfo { id: 4 },
                    map: {
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            "mood".to_string(),
                            Expr::Primary {
                                info: NodeInfo { id: 1 },
                                value: Primary::Primitive(Primitive::Identifier {
                                    info: NodeInfo { id: 0 },
                                    name: "happy".to_string(),
                                }),
                            },
                        );
                        map.insert(
                            "volume".to_string(),
                            Expr::Primary {
                                info: NodeInfo { id: 3 },
                                value: Primary::Primitive(Primitive::Identifier {
                                    info: NodeInfo { id: 2 },
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
