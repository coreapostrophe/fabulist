use pest::iterators::Pair;

use crate::{
    ast::expr::{primary::PrimaryExpr, Expr},
    parser::Rule,
};

use super::Error;

#[derive(Debug)]
pub struct LetStmt {
    pub identifier: PrimaryExpr,
    pub value: Expr,
}

impl TryFrom<Pair<'_, Rule>> for LetStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let mut inner = value.into_inner();

        let identifier = match inner
            .clone()
            .find(|pair| pair.as_rule() == Rule::identifier)
        {
            Some(identifier) => PrimaryExpr::try_from(identifier),
            None => Err(Error::map_span(value_span, "Expected identifier")),
        }?;
        let value = match inner.find(|pair| pair.as_rule() == Rule::expression) {
            Some(expression) => Expr::try_from(expression),
            None => Err(Error::map_span(value_span, "Expected value expression")),
        }?;

        Ok(LetStmt { identifier, value })
    }
}

#[cfg(test)]
mod let_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_let_stmt() {
        let test_helper = ParserTestHelper::<LetStmt>::new(Rule::let_stmt, "LetStmt");
        test_helper.assert_parse("let key = \"value\";");
    }
}
