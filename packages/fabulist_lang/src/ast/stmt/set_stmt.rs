use pest::iterators::Pair;

use crate::{
    ast::expr::{primary::PrimaryExpr, Expr},
    parser::Rule,
};

use super::Error;

#[derive(Debug)]
pub struct SetStmt {
    pub identifier: PrimaryExpr,
    pub value: Expr,
}

impl TryFrom<Pair<'_, Rule>> for SetStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let set_stmt_span = value.as_span();
        let mut inner = value.into_inner();

        let identifier = match inner.find(|pair| pair.as_rule() == Rule::identifier) {
            Some(identifier) => PrimaryExpr::try_from(identifier),
            None => Err(Error::map_span(set_stmt_span, "Expected an identifier")),
        }?;
        let value = match inner.find(|pair| pair.as_node_tag() == Some("value")) {
            Some(expression) => Expr::try_from(expression),
            None => Err(Error::map_span(set_stmt_span, "Expected value expression")),
        }?;

        Ok(SetStmt { identifier, value })
    }
}

#[cfg(test)]
mod set_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_let_stmt() {
        let test_helper = ParserTestHelper::<SetStmt>::new(Rule::set_stmt, "SetStmt");
        test_helper.assert_parse("set key = \"value\";");
    }
}
