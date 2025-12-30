use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{ast::decl::object::ObjectDecl, Parsable};

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub id: usize,
    pub object: ObjectDecl,
}

impl Parsable for Metadata {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::Story))?;

        let object = ObjectDecl::parse(parser)?;

        Ok(Metadata {
            id: parser.assign_id(),
            object,
        })
    }
}

#[cfg(test)]
mod metadata_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::object::ObjectDecl,
            expr::{literal::Literal, Expr, Primary},
            story::metadata::Metadata,
        },
        Parser,
    };

    #[test]
    fn parses_metadata() {
        let source = r#"
            Story {
                title: "My Story",
            }
        "#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let metadata = Parser::parse::<Metadata>(&tokens).expect("Failed to parse metadata");

        let expected = Metadata {
            id: 2,
            object: ObjectDecl {
                id: 1,
                map: {
                    let mut map = HashMap::new();
                    map.insert(
                        "title".to_string(),
                        Expr::Primary {
                            id: 0,
                            value: Primary::Literal(Literal::String("My Story".to_string())),
                        },
                    );
                    map
                },
            },
        };

        assert_eq!(metadata, expected);
    }
}
