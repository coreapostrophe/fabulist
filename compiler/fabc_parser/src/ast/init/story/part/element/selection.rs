use fabc_error::Error;
use fabc_lexer::tokens::TokenKind;

use crate::{ast::decl::quote::QuoteDecl, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct Selection {
    pub id: usize,
    pub choices: Vec<QuoteDecl>,
}

impl Parsable for Selection {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let mut choices = Vec::new();

        while parser.peek() == &TokenKind::Minus {
            let choice = parser.prefixed(TokenKind::Minus, |parser| QuoteDecl::parse(parser))?;
            choices.push(choice);
        }

        Ok(Selection {
            id: parser.assign_id(),
            choices,
        })
    }
}

#[cfg(test)]
mod selection_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::{object::ObjectDecl, quote::QuoteDecl},
            expr::{literal::Literal, Expr, Primary},
            init::story::part::element::selection::Selection,
        },
        Parser,
    };

    #[test]
    fn parses_selection_with_multiple_choices() {
        let source = r#"
            - "Go left." { score: 10, health: 5 }
            - "Go right." { score: 5 }
        "#;
        let tokens = Lexer::tokenize(source);
        let selection = Parser::parse_ast::<Selection>(&tokens).expect("Failed to parse selection");

        let expected = Selection {
            id: 7,
            choices: vec![
                QuoteDecl {
                    id: 3,
                    text: "Go left.".to_string(),
                    properties: Some(ObjectDecl {
                        id: 2,
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "score".to_string(),
                                Expr::Primary {
                                    id: 0,
                                    value: Primary::Literal(Literal::Number(10.0)),
                                },
                            );
                            map.insert(
                                "health".to_string(),
                                Expr::Primary {
                                    id: 1,
                                    value: Primary::Literal(Literal::Number(5.0)),
                                },
                            );
                            map
                        },
                    }),
                },
                QuoteDecl {
                    id: 6,
                    text: "Go right.".to_string(),
                    properties: Some(ObjectDecl {
                        id: 5,
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "score".to_string(),
                                Expr::Primary {
                                    id: 4,
                                    value: Primary::Literal(Literal::Number(5.0)),
                                },
                            );
                            map
                        },
                    }),
                },
            ],
        };

        assert_eq!(selection, expected);
    }
}
