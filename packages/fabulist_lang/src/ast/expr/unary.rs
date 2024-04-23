use pest::{error::LineColLocation, iterators::Pair};

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
        lcol: LineColLocation,
        operator: UnaryOperator,
        right: Expr,
    },
    Expr(Expr),
}

impl TryFrom<Pair<'_, Rule>> for UnaryExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let unary_expr_span = value.as_span();
        let unary_expr_lcol = LineColLocation::from(unary_expr_span);
        let mut inner = value.into_inner();

        if let Some(member) = inner
            .clone()
            .find(|pair| pair.as_rule() == Rule::member_expr)
        {
            Ok(UnaryExpr::Expr(Expr::try_from(member)?))
        } else {
            let operator = match inner.find(|pair| pair.as_node_tag() == Some("operator")) {
                Some(operator) => {
                    let operator_span = operator.as_span();
                    match operator.as_str() {
                        "-" => Ok(UnaryOperator::Negation),
                        "!" => Ok(UnaryOperator::Not),
                        _ => Err(Error::map_span(operator_span, "Invalid unary operator")),
                    }
                }
                None => Err(Error::map_span(unary_expr_span, "Expected unary operator")),
            }?;
            let right = match inner.find(|pair| pair.as_node_tag() == Some("right")) {
                Some(right) => Ok(Expr::try_from(right)?),
                None => Err(Error::map_span(
                    unary_expr_span,
                    "Expected value expression",
                )),
            }?;

            Ok(UnaryExpr::Unary {
                operator,
                right,
                lcol: unary_expr_lcol,
            })
        }
    }
}

impl From<UnaryExpr> for Expr {
    fn from(value: UnaryExpr) -> Self {
        if let UnaryExpr::Expr(expr) = value {
            return expr;
        }
        Expr::Unary(Box::new(value))
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
