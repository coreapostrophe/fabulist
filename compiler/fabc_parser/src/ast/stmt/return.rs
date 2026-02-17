use fabc_error::Error;
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct ReturnStmt {
    pub info: NodeInfo,
    pub value: Option<Expr>,
}

impl Parsable for ReturnStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        parser.consume(TokenKind::Keyword(KeywordKind::Return))?;
        let value = if parser.peek() != &TokenKind::Semicolon {
            Some(Expr::parse(parser)?)
        } else {
            None
        };
        parser.consume(TokenKind::Semicolon)?;
        let end_span = parser.end_span();

        Ok(ReturnStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: fabc_error::Span::from((start_span, end_span)),
            },
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::stmt::r#return::ReturnStmt, Parser};

    #[test]
    fn parses_return_with_value() {
        let source = "return 42;";
        let return_stmt =
            Parser::parse_ast_str::<ReturnStmt>(source).expect("Failed to parse return stmt");

        assert_debug_snapshot!(return_stmt);
    }

    #[test]
    fn parses_return_without_value() {
        let source = "return;";
        let return_stmt =
            Parser::parse_ast_str::<ReturnStmt>(source).expect("Failed to parse return stmt");

        assert_debug_snapshot!(return_stmt);
    }
}
