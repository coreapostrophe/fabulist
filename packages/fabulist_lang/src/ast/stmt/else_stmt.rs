use pest::iterators::Pair;

use crate::parser::Rule;

use super::{block_stmt::BlockStmt, if_stmt::IfStmt, Error};

#[derive(Debug)]
pub enum ElseStmt {
    If(IfStmt),
    Block(BlockStmt),
}

impl TryFrom<Pair<'_, Rule>> for ElseStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let mut inner = value.into_inner();

        if let Some(if_stmt) = inner.clone().find(|pair| pair.as_rule() == Rule::if_stmt) {
            Ok(ElseStmt::If(IfStmt::try_from(if_stmt)?))
        } else if let Some(block_stmt) = inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Ok(ElseStmt::Block(BlockStmt::try_from(block_stmt)?))
        } else {
            Err(Error::InvalidRule(value_rule))
        }
    }
}

#[cfg(test)]
mod else_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_else_stmt() {
        let test_helper = ParserTestHelper::<ElseStmt>::new(Rule::else_stmt, "ElseStmt");
        test_helper.assert_parse("else {}");
        test_helper.assert_parse("else if true {}");
    }
}
