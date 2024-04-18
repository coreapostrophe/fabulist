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
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_member_expr(source: &str) -> MemberExpr {
        let mut result =
            GrammarParser::parse(Rule::member_expr, source).expect("Failed to parse string.");
        let member = result.next().expect("Failed to parse member expression");
        let member_ast = MemberExpr::try_from(member);
        assert!(member_ast.is_ok());
        member_ast.expect("Failed to turn pair to `MemberExpr` struct")
    }

    #[test]
    fn parses_member_expr() {
        parse_member_expr("ident.fun().fun()");
        parse_member_expr("ident.fun(arg1, arg2).fun(arg1, arg2)");
    }
}
