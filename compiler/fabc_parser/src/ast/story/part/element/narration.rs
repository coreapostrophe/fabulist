use std::collections::HashMap;

use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::object::ObjectDecl, expr::Expr},
    expect_token, Parsable,
};

#[derive(Debug, PartialEq)]
pub struct Narration {
    pub text: String,
    pub properties: Option<HashMap<String, Expr>>,
}

impl Parsable for Narration {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
        parser.consume(TokenKind::Asterisk)?;

        let text = expect_token!(parser, TokenKind::String, "string literal")?;

        let properties = if parser.peek() == &TokenKind::LeftBrace {
            Some(ObjectDecl::parse(parser)?.map)
        } else {
            None
        };

        Ok(Narration { text, properties })
    }
}

#[cfg(test)]
mod narration_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{primitive::Primitive, Expr, Primary},
            story::part::element::narration::Narration,
        },
        Parser,
    };

    #[test]
    fn parses_narration_without_properties() {
        let source = "* \"This is a narration.\"";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let narration = Parser::parse::<Narration>(&tokens).expect("Failed to parse narration");

        let expected = Narration {
            text: "This is a narration.".to_string(),
            properties: None,
        };

        assert_eq!(narration, expected);
    }

    #[test]
    fn parses_narration_with_properties() {
        let source = "* \"This is a narration.\" { mood: happy, volume: loud }";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let narration = Parser::parse::<Narration>(&tokens).expect("Failed to parse narration");

        let expected = Narration {
            text: "This is a narration.".to_string(),
            properties: Some({
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "mood".to_string(),
                    Expr::Primary(Primary::Primitive(Primitive::Identifier(
                        "happy".to_string(),
                    ))),
                );
                map.insert(
                    "volume".to_string(),
                    Expr::Primary(Primary::Primitive(Primitive::Identifier(
                        "loud".to_string(),
                    ))),
                );
                map
            }),
        };

        assert_eq!(narration, expected);
    }
}
