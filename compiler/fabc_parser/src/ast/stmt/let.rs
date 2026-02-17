use fabc_error::{Error, Span};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct LetStmt {
    pub info: NodeInfo,
    pub name: String,
    pub initializer: Expr,
}

impl Parsable for LetStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();

        parser.consume(TokenKind::Keyword(KeywordKind::Let))?;

        let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;

        parser.consume(TokenKind::Equal)?;

        let initializer = Expr::parse(parser)?;

        parser.consume(TokenKind::Semicolon)?;

        let end_span = parser.end_span();

        Ok(LetStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            name,
            initializer,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::stmt::r#let::LetStmt, Parser};

    #[test]
    fn parses_let_statements() {
        let let_stmt =
            Parser::parse_ast_str::<LetStmt>("let x = 42;").expect("Failed to parse");

        assert_debug_snapshot!(let_stmt);
    }
}
