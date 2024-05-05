use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::stmt::block_stmt::BlockStmt, parser::Rule};

use super::{parameter_body::ParameterBodyDfn, Error};

#[derive(Debug, Clone)]
pub struct LambdaDfn {
    pub lcol: LineColLocation,
    pub parameter_body: ParameterBodyDfn,
    pub block_stmt: BlockStmt,
}

impl TryFrom<Pair<'_, Rule>> for LambdaDfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let lambda_dfn_span = value.as_span();
        let lambda_dfn_lcol = LineColLocation::from(lambda_dfn_span);
        let mut inner = value.into_inner();

        let parameter_body = match inner.find(|pair| pair.as_rule() == Rule::parameter_body) {
            Some(parameter_body_dfn) => ParameterBodyDfn::try_from(parameter_body_dfn),
            None => Err(Error::map_span(lambda_dfn_span, "Expected parameter body")),
        }?;

        let block_stmt = match inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(Error::map_span(
                lambda_dfn_span,
                "Expected a block statement",
            )),
        }?;

        Ok(LambdaDfn {
            block_stmt,
            parameter_body,
            lcol: lambda_dfn_lcol,
        })
    }
}

#[cfg(test)]
mod mutator_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_mutator() {
        let test_helper = ParserTestHelper::<LambdaDfn>::new(Rule::lambda, "LambdaDfn");
        test_helper.assert_parse("(param1, param2) => { let a = 5; }");
    }
}
