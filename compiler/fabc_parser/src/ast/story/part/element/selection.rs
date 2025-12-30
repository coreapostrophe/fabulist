use fabc_lexer::tokens::TokenKind;

use crate::{ast::decl::quote::QuoteDecl, Parsable};

#[derive(Debug, PartialEq)]
pub struct Selection {
    pub id: usize,
    pub choices: Vec<QuoteDecl>,
}

impl Parsable for Selection {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
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
            story::part::element::selection::Selection,
        },
        Parser,
    };

    #[test]
    fn parses_selection_with_multiple_choices() {
        let source = r#"
            - "Go left." { score: 10, health: 5 }
            - "Go right." { score: 5 }
        "#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let selection = Parser::parse::<Selection>(&tokens).expect("Failed to parse selection");

        let expected = Selection {
            id: 4,
            choices: vec![
                QuoteDecl {
                    id: 1,
                    text: "Go left.".to_string(),
                    properties: Some(ObjectDecl {
                        id: 0,
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "score".to_string(),
                                Expr::Primary(Primary::Literal(Literal::Number(10.0))),
                            );
                            map.insert(
                                "health".to_string(),
                                Expr::Primary(Primary::Literal(Literal::Number(5.0))),
                            );
                            map
                        },
                    }),
                },
                QuoteDecl {
                    id: 3,
                    text: "Go right.".to_string(),
                    properties: Some(ObjectDecl {
                        id: 2,
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "score".to_string(),
                                Expr::Primary(Primary::Literal(Literal::Number(5.0))),
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
