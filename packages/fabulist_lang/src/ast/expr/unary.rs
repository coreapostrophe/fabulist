use pest::iterators::Pair;

use crate::parser::Rule;

use super::{Error, Expr};

#[derive(Debug)]
pub enum UnaryOperator {
    Negation,
    Not,
}

impl TryFrom<String> for UnaryOperator {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "-" => Ok(UnaryOperator::Negation),
            "!" => Ok(UnaryOperator::Not),
            _ => Err(Error::InvalidUnaryOperator),
        }
    }
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
        let value_rule = value.as_rule();
        let inner = value.into_inner();

        let member = inner
            .clone()
            .find(|pair| pair.as_rule() == Rule::member_expr);

        if let Some(member) = member {
            Ok(UnaryExpr::Expr(Expr::try_from(member)?))
        } else {
            let operator = match inner.find_first_tagged("operator") {
                Some(operator) => Ok(UnaryOperator::try_from(operator.as_str().to_string())?),
                None => Err(Error::InvalidRule(value_rule)),
            }?;
            let right = match inner.find_first_tagged("right") {
                Some(right) => Ok(Expr::try_from(right)?),
                None => Err(Error::InvalidRule(value_rule)),
            }?;

            Ok(UnaryExpr::Unary { operator, right })
        }
    }
}

#[cfg(test)]
mod unary_expr_tests {
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_unary_expr(source: &str) -> UnaryExpr {
        let mut result =
            GrammarParser::parse(Rule::unary_expr, source).expect("Failed to parse string.");
        let unary = result.next().expect("Failed to parse unary expression");
        let unary_ast = UnaryExpr::try_from(unary);
        assert!(unary_ast.is_ok());
        unary_ast.expect("Failed to turn pair to `UnaryExpr` struct")
    }

    #[test]
    fn parses_unary_expr() {
        parse_unary_expr("!5");
        parse_unary_expr("!(true)");
        parse_unary_expr("!!!ident");
        parse_unary_expr("-\"num\"");
    }
}
