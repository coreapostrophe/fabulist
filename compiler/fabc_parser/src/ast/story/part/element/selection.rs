use fabc_lexer::tokens::TokenKind;

use crate::{ast::story::part::element::selection::choice::Choice, Parsable};

pub mod choice;

#[derive(Debug, PartialEq)]
pub struct Selection {
    pub choices: Vec<Choice>,
}

impl Parsable for Selection {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        let mut choices = Vec::new();
        while parser.peek() == &TokenKind::Minus {
            let choice = Choice::parse(parser)?;
            choices.push(choice);
        }

        Ok(Selection { choices })
    }
}

#[cfg(test)]
mod selection_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            story::part::element::selection::{choice::Choice, Selection},
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
            choices: vec![
                Choice {
                    text: "Go left.".to_string(),
                    properties: Some({
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
                    }),
                },
                Choice {
                    text: "Go right.".to_string(),
                    properties: Some({
                        let mut map = HashMap::new();
                        map.insert(
                            "score".to_string(),
                            Expr::Primary(Primary::Literal(Literal::Number(5.0))),
                        );
                        map
                    }),
                },
            ],
        };

        assert!(selection == expected);
    }
}
