use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::expr::Expr, parser::Rule};

use super::{block_stmt::BlockStmt, else_stmt::ElseStmt, Error};

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub lcol: LineColLocation,
    pub condition: Expr,
    pub block_stmt: BlockStmt,
    pub else_stmt: Option<Box<ElseStmt>>,
}

impl TryFrom<Pair<'_, Rule>> for IfStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let if_stmt_span = value.as_span();
        let if_stmt_lcol = LineColLocation::from(if_stmt_span);
        let mut inner = value.into_inner();

        let condition = match inner.find(|pair| pair.as_node_tag() == Some("condition")) {
            Some(condition) => Expr::try_from(condition),
            None => Err(Error::map_span(
                if_stmt_span,
                "Expected condition expression",
            )),
        }?;
        let block_stmt = match inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(Error::map_span(if_stmt_span, "Expected block statement")),
        }?;
        let else_stmt = match inner.find(|pair| pair.as_rule() == Rule::else_stmt) {
            Some(else_stmt) => Some(Box::new(ElseStmt::try_from(else_stmt)?)),
            None => None,
        };

        Ok(IfStmt {
            condition,
            block_stmt,
            else_stmt,
            lcol: if_stmt_lcol,
        })
    }
}

#[cfg(test)]
mod if_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_if_stmt() {
        let test_helper = ParserTestHelper::<IfStmt>::new(Rule::if_stmt, "IfStmt");
        test_helper.assert_parse("if true {}");
        test_helper.assert_parse("if true {} else {}");
    }
}
