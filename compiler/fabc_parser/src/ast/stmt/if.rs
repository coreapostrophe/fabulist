use fabc_error::{Error, Span};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, stmt::block::BlockStmt, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub enum ElseClause {
    If(Box<IfStmt>),
    Block(Box<BlockStmt>),
}

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    pub info: NodeInfo,
    pub condition: Expr,
    pub then_branch: Box<BlockStmt>,
    pub else_branch: Option<ElseClause>,
}

impl Parsable for IfStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();

        parser.consume(TokenKind::Keyword(KeywordKind::If))?;

        let condition = parser.enclosed(TokenKind::LeftParen, TokenKind::RightParen, |parser| {
            Expr::parse(parser)
        })?;

        let then_branch = Box::new(BlockStmt::parse(parser)?);

        let else_branch = if parser.r#match(&[TokenKind::Keyword(KeywordKind::Else)]) {
            if parser.r#match(&[TokenKind::Keyword(KeywordKind::If)]) {
                Some(ElseClause::If(Box::new(IfStmt::parse(parser)?)))
            } else {
                Some(ElseClause::Block(Box::new(BlockStmt::parse(parser)?)))
            }
        } else {
            None
        };

        let end_span = parser.end_span();

        Ok(IfStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            condition,
            then_branch,
            else_branch,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::stmt::r#if::IfStmt, Parser};

    #[test]
    fn parses_if_stmt_without_else() {
        let if_stmt =
            Parser::parse_ast_str::<IfStmt>("if (true) { }").expect("Failed to parse if statement");

        assert_debug_snapshot!(if_stmt);
    }

    #[test]
    fn parses_if_stmt_with_else_block() {
        let if_stmt = Parser::parse_ast_str::<IfStmt>("if (false) { } else { }")
            .expect("Failed to parse if statement");

        assert_debug_snapshot!(if_stmt);
    }
}
