use fabc_error::Error;
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{decl::object::ObjectDecl, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub info: NodeInfo,
    pub object: ObjectDecl,
}

impl Parsable for Metadata {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::Story))?;

        let object = ObjectDecl::parse(parser)?;

        Ok(Metadata {
            info: NodeInfo {
                id: parser.assign_id(),
            },
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
            init::story::metadata::Metadata,
            NodeInfo,
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
        let tokens = Lexer::tokenize(source);
        let metadata = Parser::parse_ast::<Metadata>(&tokens).expect("Failed to parse metadata");

        let expected = Metadata {
            info: NodeInfo { id: 2 },
            object: ObjectDecl {
                info: NodeInfo { id: 1 },
                map: {
                    let mut map = HashMap::new();
                    map.insert(
                        "title".to_string(),
                        Expr::Primary {
                            info: NodeInfo { id: 0 },
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
