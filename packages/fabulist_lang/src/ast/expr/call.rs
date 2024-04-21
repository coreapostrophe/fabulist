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
        let call_expr_span = value.as_span();
        let mut inner = value.into_inner();

        let callee = match inner.find(|pair| pair.as_node_tag() == Some("callee")) {
            Some(callee) => Expr::try_from(callee),
            None => Err(Error::map_span(
                call_expr_span,
                "Expected a callee expression",
            )),
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

impl From<CallExpr> for Expr {
    fn from(value: CallExpr) -> Self {
        if value.argument_body.is_none() {
            return value.callee;
        }
        Expr::Call(Box::new(value))
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
