use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::stmt::block_stmt::BlockStmt, parser::Rule};

use super::Error;

#[derive(Debug, Clone)]
pub struct MutatorDfn {
    pub lcol: LineColLocation,
    pub block_stmt: BlockStmt,
}

impl TryFrom<Pair<'_, Rule>> for MutatorDfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mutator_dfn_span = value.as_span();
        let mutator_dfn_lcol = LineColLocation::from(mutator_dfn_span);

        let block_stmt = match value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::block_stmt)
        {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(Error::map_span(
                mutator_dfn_span,
                "Expected a block statement",
            )),
        }?;

        Ok(MutatorDfn {
            block_stmt,
            lcol: mutator_dfn_lcol,
        })
    }
}

#[cfg(test)]
mod mutator_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_mutator() {
        let test_helper = ParserTestHelper::<MutatorDfn>::new(Rule::mutator, "MutatorDfn");
        test_helper.assert_parse("|>{}");
    }
}
