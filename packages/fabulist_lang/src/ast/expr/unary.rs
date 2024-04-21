use pest::iterators::Pair;

use crate::parser::Rule;

use super::{Error, Expr};

#[derive(Debug)]
pub enum UnaryOperator {
    Negation,
    Not,
}

#[derive(Debug)]
pub enum UnaryExpr {
    Unary {
        operator: UnaryOperator,
        right: Expr,
    },
    Expr(Expr),
}

impl TryFrom<Pair<'_, Rule>> for UnaryExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let inner = value.into_inner();

        let member = inner
            .clone()
            .find(|pair| pair.as_rule() == Rule::member_expr);

        if let Some(member) = member {
            Ok(UnaryExpr::Expr(Expr::try_from(member)?))
        } else {
            let operator = match inner.find_first_tagged("operator") {
                Some(operator) => {
                    let operator_span = operator.as_span();
                    match operator.as_str() {
                        "-" => Ok(UnaryOperator::Negation),
                        "!" => Ok(UnaryOperator::Not),
                        _ => Err(Error::map_span(operator_span, "Invalid unary operator")),
                    }
                }
                None => Err(Error::map_span(value_span, "Expected unary operator")),
            }?;
            let right = match inner.find_first_tagged("right") {
                Some(right) => Ok(Expr::try_from(right)?),
                None => Err(Error::map_span(value_span, "Expected value expression")),
            }?;

            Ok(UnaryExpr::Unary { operator, right })
        }
    }
}

#[cfg(test)]
mod unary_expr_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_unary_expr() {
        let test_helper = ParserTestHelper::<UnaryExpr>::new(Rule::unary_expr, "UnaryExpr");
        test_helper.assert_parse("!5");
        test_helper.assert_parse("!(true)");
        test_helper.assert_parse("!!!ident");
        test_helper.assert_parse("-\"num\"");
    }
}
