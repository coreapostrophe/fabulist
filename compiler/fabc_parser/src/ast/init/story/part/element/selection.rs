use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::quote::QuoteDecl, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct SelectionElement {
    pub info: NodeInfo,
    pub choices: Vec<QuoteDecl>,
}

impl Parsable for SelectionElement {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        let mut choices = Vec::new();

        while parser.peek() == &TokenKind::Minus {
            let choice = parser.prefixed(TokenKind::Minus, |parser| QuoteDecl::parse(parser))?;
            choices.push(choice);
        }
        let end_span = parser.end_span();

        Ok(SelectionElement {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            choices,
        })
    }
}

#[cfg(test)]
mod selection_tests {
    use std::collections::HashMap;

    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::{object::ObjectDecl, quote::QuoteDecl},
            expr::{literal::Literal, Expr, Primary},
            init::story::part::element::selection::SelectionElement,
            NodeInfo,
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
        let selection =
            Parser::parse_ast::<SelectionElement>(&tokens).expect("Failed to parse selection");

        let expected = SelectionElement {
            info: NodeInfo {
                id: 7,
                span: Span::from((LineCol::new(2, 13), LineCol::new(3, 38))),
            },
            choices: vec![
                QuoteDecl {
                    info: NodeInfo {
                        id: 3,
                        span: Span::from((LineCol::new(2, 15), LineCol::new(2, 49))),
                    },
                    text: "Go left.".to_string(),
                    properties: Some(ObjectDecl {
                        info: NodeInfo {
                            id: 2,
                            span: Span::from((LineCol::new(2, 26), LineCol::new(2, 49))),
                        },
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "score".to_string(),
                                Expr::Primary {
                                    info: NodeInfo {
                                        id: 0,
                                        span: Span::from((
                                            LineCol::new(2, 35),
                                            LineCol::new(2, 36),
                                        )),
                                    },
                                    value: Primary::Literal(Literal::Number(10.0)),
                                },
                            );
                            map.insert(
                                "health".to_string(),
                                Expr::Primary {
                                    info: NodeInfo {
                                        id: 1,
                                        span: Span::from((
                                            LineCol::new(2, 47),
                                            LineCol::new(2, 47),
                                        )),
                                    },
                                    value: Primary::Literal(Literal::Number(5.0)),
                                },
                            );
                            map
                        },
                    }),
                },
                QuoteDecl {
                    info: NodeInfo {
                        id: 6,
                        span: Span::from((LineCol::new(3, 15), LineCol::new(3, 38))),
                    },
                    text: "Go right.".to_string(),
                    properties: Some(ObjectDecl {
                        info: NodeInfo {
                            id: 5,
                            span: Span::from((LineCol::new(3, 27), LineCol::new(3, 38))),
                        },
                        map: {
                            let mut map = HashMap::new();
                            map.insert(
                                "score".to_string(),
                                Expr::Primary {
                                    info: NodeInfo {
                                        id: 4,
                                        span: Span::from((
                                            LineCol::new(3, 36),
                                            LineCol::new(3, 36),
                                        )),
                                    },
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
