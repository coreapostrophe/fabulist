use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::dfn::path::PathDfn, parser::Rule};

use super::Error;

#[derive(Debug, Clone)]
pub struct GotoStmt {
    pub lcol: LineColLocation,
    pub path: PathDfn,
}

impl TryFrom<Pair<'_, Rule>> for GotoStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let goto_stmt_span = value.as_span();
        let goto_stmt_lcol = LineColLocation::from(goto_stmt_span);

        let path = match value.into_inner().find(|pair| pair.as_rule() == Rule::path) {
            Some(path) => PathDfn::try_from(path),
            None => Err(Error::map_span(goto_stmt_span, "Expected path expression")),
        }?;

        Ok(GotoStmt {
            path,
            lcol: goto_stmt_lcol,
        })
    }
}

#[cfg(test)]
mod goto_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_goto_stmt() {
        let test_helper = ParserTestHelper::<GotoStmt>::new(Rule::goto_stmt, "GotoStmt");
        test_helper.assert_parse("goto module_1::part_1;");
    }
}
