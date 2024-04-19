use pest::iterators::Pair;

use crate::parser::Rule;

use super::{Error, Expr};

#[derive(Debug)]
pub struct MemberExpr {
    pub left: Expr,
    pub members: Vec<Expr>,
}

impl TryFrom<Pair<'_, Rule>> for MemberExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let mut inner = value.into_inner();

        let left = match inner.next() {
            Some(left) => Expr::try_from(left),
            None => Err(Error::InvalidRule(value_rule)),
        }?;
        let members = inner
            .map(|pair| Expr::try_from(pair))
            .collect::<Result<Vec<Expr>, Error>>()?;

        Ok(MemberExpr { left, members })
    }
}

#[cfg(test)]
mod member_expr_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_member_expr() {
        let test_helper = ParserTestHelper::<MemberExpr>::new(Rule::member_expr, "MemberExpr");
        test_helper.assert_parse("ident.fun().fun()");
        test_helper.assert_parse("ident.fun(arg1, arg2).fun(arg1, arg2)");
    }
}
