use pest::iterators::Pair;

use crate::{ast::dfn::argument_body::ArgumentBody, parser::Rule};

use super::{Error, Expr};

#[derive(Debug)]
pub struct CallExpr {
    pub callee: Expr,
    pub argument_body: Option<ArgumentBody>,
}

impl TryFrom<Pair<'_, Rule>> for CallExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let mut inner = value.into_inner();

        let callee = match inner.find_first_tagged("callee") {
            Some(callee) => Ok(Expr::try_from(callee)?),
            None => Err(Error::InvalidRule(value_rule)),
        }?;
        let argument_body = match inner.find(|pair| pair.as_rule() == Rule::argument_body) {
            Some(argument_body) => Some(ArgumentBody::try_from(argument_body)?),
            None => None,
        };

        Ok(CallExpr {
            callee,
            argument_body,
        })
    }
}

#[cfg(test)]
mod call_expr_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_call_expr() {
        let test_helper = ParserTestHelper::<CallExpr>::new(Rule::call_expr, "CallExpr");
        test_helper.assert_parse("test()");
        test_helper.assert_parse("5()");
        test_helper.assert_parse("\"Yo\"()");
        test_helper.assert_parse("false()");
    }
}
