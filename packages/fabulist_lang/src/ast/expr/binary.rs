use pest::{error::LineColLocation, iterators::Pair};

use crate::parser::Rule;

use super::{Error, Expr};

#[derive(Debug)]
pub enum BinaryOperator {
    Divide,
    Multiply,
    Addition,
    Subtraction,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
    EqualEqual,
    NotEqual,
    And,
    Or,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lcol: LineColLocation,
    pub left: Expr,
    pub operator: Option<BinaryOperator>,
    pub right: Option<Expr>,
}

impl TryFrom<Pair<'_, Rule>> for BinaryExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let binary_expr_span = value.as_span();
        let binary_expr_lcol = LineColLocation::from(binary_expr_span);
        let mut inner = value.into_inner();

        let left = match inner.find(|pair| pair.as_node_tag() == Some("left")) {
            Some(left) => Expr::try_from(left),
            None => Err(Error::map_span(
                binary_expr_span,
                "Expected a value expression",
            )),
        }?;
        let operator = match inner.find(|pair| pair.as_node_tag() == Some("operator")) {
            Some(operator) => {
                let operator_span = operator.as_span();
                Some(match operator.as_str() {
                    "/" => Ok(BinaryOperator::Divide),
                    "*" => Ok(BinaryOperator::Multiply),
                    "+" => Ok(BinaryOperator::Addition),
                    "-" => Ok(BinaryOperator::Subtraction),
                    ">" => Ok(BinaryOperator::GreaterThan),
                    ">=" => Ok(BinaryOperator::GreaterEqual),
                    "<" => Ok(BinaryOperator::LessThan),
                    "<=" => Ok(BinaryOperator::LessEqual),
                    "==" => Ok(BinaryOperator::EqualEqual),
                    "!=" => Ok(BinaryOperator::NotEqual),
                    "&&" => Ok(BinaryOperator::And),
                    "||" => Ok(BinaryOperator::Or),
                    _ => Err(Error::map_span(operator_span, "Invalid binary operator")),
                }?)
            }
            None => None,
        };
        let right = match inner.find(|pair| pair.as_node_tag() == Some("right")) {
            Some(right) => Some(Expr::try_from(right)?),
            None => None,
        };

        Ok(BinaryExpr {
            left,
            operator,
            right,
            lcol: binary_expr_lcol,
        })
    }
}

impl From<BinaryExpr> for Expr {
    fn from(value: BinaryExpr) -> Self {
        if value.operator.is_none() && value.right.is_none() {
            return value.left;
        }
        Expr::Binary(Box::new(value))
    }
}

#[cfg(test)]
mod binary_expr_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_binary_expr() {
        let test_helper = ParserTestHelper::<BinaryExpr>::new(Rule::logical_expr, "BinaryExpr");
        test_helper.assert_parse("5 + 2");
        test_helper.assert_parse("5/ 2");
        test_helper.assert_parse("5 *2");
        test_helper.assert_parse("5== 2");
    }
}
