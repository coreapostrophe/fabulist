use pest::iterators::Pair;

use crate::parser::Rule;

use super::{Error, Stmt};

#[derive(Debug)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

impl TryFrom<Pair<'_, Rule>> for BlockStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let statements = value
            .into_inner()
            .map(|pair| Stmt::try_from(pair))
            .collect::<Result<Vec<Stmt>, Error>>()?;

        Ok(BlockStmt { statements })
    }
}

#[cfg(test)]
mod block_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_block_stmt() {
        let test_helper = ParserTestHelper::<BlockStmt>::new(Rule::block_stmt, "BlockStmt");
        test_helper.assert_parse("{}");
    }
}
