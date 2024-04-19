use pest::iterators::Pair;

use crate::{ast::expr::Expr, parser::Rule};

use super::{block_stmt::BlockStmt, else_stmt::ElseStmt, Error};

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub block_stmt: BlockStmt,
    pub else_stmt: Option<Box<ElseStmt>>,
}

impl TryFrom<Pair<'_, Rule>> for IfStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let mut inner = value.into_inner();

        let condition = match inner
            .clone()
            .find(|pair| pair.as_node_tag() == Some("condition"))
        {
            Some(condition) => Expr::try_from(condition),
            None => Err(Error::InvalidRule(value_rule)),
        }?;
        let block_stmt = match inner
            .clone()
            .find(|pair| pair.as_rule() == Rule::block_stmt)
        {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(Error::InvalidRule(value_rule)),
        }?;
        let else_stmt = match inner.find(|pair| pair.as_rule() == Rule::else_stmt) {
            Some(else_stmt) => Some(Box::new(ElseStmt::try_from(else_stmt)?)),
            None => None,
        };

        Ok(IfStmt {
            condition,
            block_stmt,
            else_stmt,
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
    }
}
