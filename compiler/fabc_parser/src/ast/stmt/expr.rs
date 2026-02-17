use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{expr::Expr, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct ExprStmt {
    pub info: NodeInfo,
    pub expr: Expr,
}

impl Parsable for ExprStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        let expr = Expr::parse(parser)?;
        parser.consume(TokenKind::Semicolon)?;
        let end_span = parser.end_span();

        Ok(ExprStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            expr,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::stmt::expr::ExprStmt, Parser};

    #[test]
    fn parses_expr_statements() {
        let expr_stmt = Parser::parse_ast_str::<ExprStmt>("x + 1;")
            .expect("Failed to parse expr statement");

        assert_debug_snapshot!(expr_stmt);
    }
}
