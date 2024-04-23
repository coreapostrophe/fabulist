use pest::{error::LineColLocation, iterators::Pair};

use crate::parser::Rule;

use super::{Error, Expr};

#[derive(Debug)]
pub struct MemberExpr {
    pub lcol: LineColLocation,
    pub left: Expr,
    pub members: Vec<Expr>,
}

impl TryFrom<Pair<'_, Rule>> for MemberExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let member_expr_span = value.as_span();
        let member_expr_lcol = LineColLocation::from(member_expr_span);
        let mut inner = value.into_inner();

        let left = match inner.next() {
            Some(left) => Expr::try_from(left),
            None => Err(Error::map_span(
                member_expr_span,
                "Expected a value expression",
            )),
        }?;
        let members = inner
            .map(|pair| Expr::try_from(pair))
            .collect::<Result<Vec<Expr>, Error>>()?;

        Ok(MemberExpr {
            left,
            members,
            lcol: member_expr_lcol,
        })
    }
}

impl From<MemberExpr> for Expr {
    fn from(value: MemberExpr) -> Self {
        if value.members.is_empty() {
            return value.left;
        }
        Expr::Member(Box::new(value))
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
