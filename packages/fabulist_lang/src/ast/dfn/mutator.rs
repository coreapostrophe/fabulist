use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::stmt::block_stmt::BlockStmt, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct Mutator {
    pub lcol: LineColLocation,
    pub block_stmt: BlockStmt,
}

impl TryFrom<Pair<'_, Rule>> for Mutator {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mutator_span = value.as_span();
        let mutator_lcol = LineColLocation::from(mutator_span);

        let block_stmt = match value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::block_stmt)
        {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(Error::map_span(mutator_span, "Expected a block statement")),
        }?;

        Ok(Mutator {
            block_stmt,
            lcol: mutator_lcol,
        })
    }
}

#[cfg(test)]
mod mutator_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_mutator() {
        let test_helper = ParserTestHelper::<Mutator>::new(Rule::mutator, "Mutator");
        test_helper.assert_parse("|>{}");
    }
}
