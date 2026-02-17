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
    use insta::assert_debug_snapshot;

    use crate::{ast::init::story::metadata::Metadata, Parser};

    #[test]
    fn parses_metadata() {
        let metadata = Parser::parse_ast_str::<Metadata>(
            r#"
            Story {
                title: "My Story",
            }
        "#,
        )
        .expect("Failed to parse metadata");

        assert_debug_snapshot!(metadata);
    }
}
