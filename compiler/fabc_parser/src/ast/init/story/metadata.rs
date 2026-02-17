use fabc_error::{Error, Span};
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
        let start_span = parser.start_span();
        parser.consume(TokenKind::Keyword(KeywordKind::Story))?;
        let object = ObjectDecl::parse(parser)?;
        let end_span = parser.end_span();

        Ok(Metadata {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            object,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fabc_error::{LineCol, Span};
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
            info: NodeInfo {
                id: 3,
                span: Span::from((LineCol::new(2, 13), LineCol::new(4, 13))),
            },
            object: ObjectDecl {
                info: NodeInfo {
                    id: 2,
                    span: Span::from((LineCol::new(2, 19), LineCol::new(4, 13))),
                },
                map: {
                    let mut map = HashMap::new();
                    map.insert(
                        "title".to_string(),
                        Expr::Primary {
                            info: NodeInfo {
                                id: 1,
                                span: Span::from((LineCol::new(3, 24), LineCol::new(3, 33))),
                            },
                            value: Primary::Literal(Literal::String {
                                info: NodeInfo {
                                    id: 0,
                                    span: Span::from((LineCol::new(3, 24), LineCol::new(3, 33))),
                                },
                                value: "My Story".to_string(),
                            }),
                        },
                    );
                    map
                },
            },
        };

        assert_eq!(metadata, expected);
    }
}
