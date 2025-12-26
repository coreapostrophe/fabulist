use std::collections::HashMap;

use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::object::ObjectDecl, expr::Expr},
    expect_token, Parsable,
};

#[derive(Debug, PartialEq)]
pub struct Choice {
    pub text: String,
    pub properties: Option<HashMap<String, Expr>>,
}

impl Parsable for Choice {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
        parser.consume(TokenKind::Minus)?;

        let text = expect_token!(parser, TokenKind::String, "choice text")?;

        let properties = if parser.peek() == &TokenKind::LeftBrace {
            Some(ObjectDecl::parse(parser)?.map)
        } else {
            None
        };

        Ok(Choice { text, properties })
    }
}

#[cfg(test)]
mod choice_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            story::part::element::selection::choice::Choice,
        },
        Parser,
    };

    #[test]
    fn parses_choice_without_properties() {
        let source = r#"- "Go left.""#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let choice = Parser::parse::<Choice>(&tokens).expect("Failed to parse choice");

        let expected = Choice {
            text: "Go left.".to_string(),
            properties: None,
        };

        assert_eq!(choice, expected);
    }

    #[test]
    fn parses_choice_with_properties() {
        let source = r#"- "Go right." { score: 10, health: 5 }"#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let choice = Parser::parse::<Choice>(&tokens).expect("Failed to parse choice");

        let expected = Choice {
            text: "Go right.".to_string(),
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
        };
        assert_eq!(choice, expected);
    }
}
