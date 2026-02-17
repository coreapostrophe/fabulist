use std::collections::BTreeMap;

use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{expr::Expr, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct ObjectDecl {
    pub info: NodeInfo,
    pub map: BTreeMap<String, Expr>,
}

impl Parsable for ObjectDecl {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        let map = {
            let punctuated_vec = parser.punctuated(
                TokenKind::LeftBrace,
                TokenKind::RightBrace,
                TokenKind::Comma,
                |parser| {
                    let key = expect_token!(parser, TokenKind::Identifier, "identifier")?;
                    parser.consume(TokenKind::Colon)?;
                    let value = Expr::parse(parser)?;
                    Ok((key, value))
                },
            )?;
            punctuated_vec
                .into_iter()
                .collect::<BTreeMap<String, Expr>>()
        };
        let end_span = parser.end_span();

        Ok(ObjectDecl {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            map,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::decl::object::ObjectDecl, Parser};

    #[test]
    fn parses_object_decl() {
        let object_decl = Parser::parse_ast_str::<ObjectDecl>(
            r#"
            {
                key1: "value1",
                key2: 42
            }
        "#,
        )
        .expect("Failed to parse object declaration");

        assert_debug_snapshot!(object_decl);
    }
}
