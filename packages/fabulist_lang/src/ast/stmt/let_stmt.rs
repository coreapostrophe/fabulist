use pest::{error::LineColLocation, iterators::Pair};

use crate::{
    ast::expr::{primary::PrimaryExpr, Expr},
    parser::Rule,
};

use super::Error;

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub lcol: LineColLocation,
    pub identifier: PrimaryExpr,
    pub value: Expr,
}

impl TryFrom<Pair<'_, Rule>> for LetStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let let_stmt_span = value.as_span();
        let let_stmt_lcol = LineColLocation::from(let_stmt_span);
        let mut inner = value.into_inner();

        let identifier = match inner.find(|pair| pair.as_rule() == Rule::identifier) {
            Some(identifier) => PrimaryExpr::try_from(identifier),
            None => Err(Error::map_span(let_stmt_span, "Expected an identifier")),
        }?;
        let value = match inner.find(|pair| pair.as_node_tag() == Some("value")) {
            Some(expression) => Expr::try_from(expression),
            None => Err(Error::map_span(let_stmt_span, "Expected value expression")),
        }?;

        Ok(LetStmt {
            identifier,
            value,
            lcol: let_stmt_lcol,
        })
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
