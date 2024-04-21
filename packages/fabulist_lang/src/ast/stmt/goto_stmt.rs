use pest::iterators::Pair;

use crate::{ast::dfn::path::Path, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct GotoStmt {
    pub path: Path,
}

impl TryFrom<Pair<'_, Rule>> for GotoStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        let path = match value.into_inner().find(|pair| pair.as_rule() == Rule::path) {
            Some(path) => Path::try_from(path),
            None => Err(Error::map_span(value_span, "Expected path")),
        }?;

        Ok(GotoStmt { path })
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
