use std::collections::HashMap;

use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{
    ast::expr::{primitive::Primitive, Expr},
    error::Error,
    Parsable,
};

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub map: HashMap<String, Expr>,
}

impl Parsable for Metadata {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(Token::Keyword(KeywordKind::Story))?;

        let map_vec = parser.punctuated(
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            |parser| {
                let Primitive::Identifier(key) = Primitive::parse(parser)? else {
                    return Err(Error::ExpectedFound {
                        expected: "identifier".to_string(),
                        found: parser.peek().to_string(),
                    });
                };
                parser.consume(Token::Colon)?;
                let value = Expr::parse(parser)?;

                Ok((key, value))
            },
        );

        let mut map = HashMap::new();
        for (key, value) in map_vec? {
            map.insert(key, value);
        }

        Ok(Metadata { map })
    }
}

#[cfg(test)]
mod metadata_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            story::metadata::Metadata,
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_metadat() {
        let source = r#"
            Story {
                title: "My Story",
            }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");
        let mut parser = Parser::new(tokens);

        let metadata = Metadata::parse(&mut parser).expect("Failed to parse metadata");

        let expected_map = {
            let mut map = HashMap::new();
            map.insert(
                "title".to_string(),
                Expr::Primary(Primary::Literal(Literal::String("My Story".to_string()))),
            );
            map
        };

        assert_eq!(metadata.map, expected_map);
    }
}
