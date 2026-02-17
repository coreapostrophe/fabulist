use fabc_error::{Error, Span};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct GotoStmt {
    pub info: NodeInfo,
    pub target: Box<Expr>,
}

impl Parsable for GotoStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        parser.consume(TokenKind::Keyword(KeywordKind::Goto))?;
        let target = Expr::parse(parser)?;
        parser.consume(TokenKind::Semicolon)?;
        let end_span = parser.end_span();

        Ok(GotoStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            target: Box::new(target),
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::stmt::goto::GotoStmt, Parser};

    #[test]
    fn parses_goto_statements() {
        let goto_stmt = Parser::parse_ast_str::<GotoStmt>("goto module_ns.part_ident;")
            .expect("Failed to parse goto statement");

        assert_debug_snapshot!(goto_stmt);
    }
}
