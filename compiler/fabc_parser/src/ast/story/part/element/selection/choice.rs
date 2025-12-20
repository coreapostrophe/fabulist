use std::collections::HashMap;

use fabc_lexer::tokens::Token;

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
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(Token::Minus)?;

        let text = expect_token!(parser, Token::String, "choice text")?;

        let properties = if parser.peek() == &Token::LeftBrace {
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
        Parsable, Parser,
    };

    #[test]
    fn parses_choice_without_properties() {
        let source = r#"- "Go left.""#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(&tokens);
        let choice = Choice::parse(&mut parser).expect("Failed to parse choice");

        let expected = Choice {
            text: "Go left.".to_string(),
            properties: None,
        };

        assert_eq!(choice, expected);
    }

    #[test]
    fn parses_choice_with_properties() {
        let source = r#"- "Go right." { score: 10, health: 5 }"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");

        let mut parser = Parser::new(&tokens);
        let choice = Choice::parse(&mut parser).expect("Failed to parse choice");

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
