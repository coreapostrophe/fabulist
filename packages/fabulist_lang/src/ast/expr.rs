use fabulist_derive::SyntaxTree;
use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::dfn::ArgumentBodyDfn, error::Error, parser::Rule};

use self::primary::Primary;

pub mod literal;
pub mod primary;
pub mod primitive;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negation,
    Not,
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Unary {
    #[production(operator: UnaryOperator, right: Expr)]
    Standard(StandardUnary),
    #[production(expr: Expr)]
    Pass(PassUnary),
}

impl TryFrom<Pair<'_, Rule>> for Unary {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let mut inner = value.into_inner();

        if let Some(member) = inner
            .clone()
            .find(|pair| pair.as_rule() == Rule::member_expr)
        {
            Ok(Unary::Pass(PassUnary {
                lcol: value_lcol,
                expr: Expr::try_from(member)?,
            }))
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
                None => Err(Error::map_span(value_span, "Expected unary operator")),
            }?;
            let right = match inner.find(|pair| pair.as_node_tag() == Some("right")) {
                Some(right) => Ok(Expr::try_from(right)?),
                None => Err(Error::map_span(value_span, "Expected value expression")),
            }?;

            Ok(Unary::Standard(StandardUnary {
                lcol: value_lcol,
                operator,
                right,
            }))
        }
    }
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Expr {
    #[production(primary: Primary)]
    Primary(Box<PrimaryExpr>),

    #[production(unary: Unary)]
    Unary(Box<UnaryExpr>),

    #[production(callee: Expr, argument_body: Option<ArgumentBodyDfn>)]
    Call(Box<CallExpr>),

    #[production(left: Expr, members: Vec<Expr>)]
    Member(Box<MemberExpr>),

    #[production(left: Expr, operator: Option<BinaryOperator>, right: Option<Expr>)]
    Binary(Box<BinaryExpr>),
}

impl TryFrom<Pair<'_, Rule>> for Expr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        match value.as_rule() {
            Rule::expression => match value.into_inner().next() {
                Some(inner) => Ok(Expr::try_from(inner)?),
                None => Err(Error::map_span(
                    value_span,
                    "Unable to parse token tree interior",
                )),
            },

            Rule::unary_expr => Ok(UnaryExpr::try_from(value)?.into()),
            Rule::call_expr => Ok(CallExpr::try_from(value)?.into()),
            Rule::member_expr => Ok(MemberExpr::try_from(value)?.into()),

            Rule::logical_expr
            | Rule::equality_expr
            | Rule::comparison_expr
            | Rule::term_expr
            | Rule::factor_expr => Ok(BinaryExpr::try_from(value)?.into()),

            Rule::primary_expr
            | Rule::number
            | Rule::identifier
            | Rule::strict_ident
            | Rule::raw_ident
            | Rule::string
            | Rule::raw_string
            | Rule::path
            | Rule::object
            | Rule::lambda
            | Rule::grouping
            | Rule::boolean
            | Rule::none => Ok(PrimaryExpr::try_from(value)?.into()),
            _ => Err(Error::map_span(value_span, "Invalid expression")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for PrimaryExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());
        Ok(PrimaryExpr {
            lcol: value_lcol,
            primary: Primary::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for UnaryExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());
        Ok(UnaryExpr {
            lcol: value_lcol,
            unary: Unary::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for CallExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let mut inner = value.into_inner();

        let callee = match inner.find(|pair| pair.as_node_tag() == Some("callee")) {
            Some(callee) => Expr::try_from(callee),
            None => Err(Error::map_span(value_span, "Expected a callee expression")),
        }?;
        let argument_body = match inner.find(|pair| pair.as_rule() == Rule::argument_body) {
            Some(argument_body) => Some(ArgumentBodyDfn::try_from(argument_body)?),
            None => None,
        };

        Ok(CallExpr {
            callee,
            argument_body,
            lcol: value_lcol,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for MemberExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let mut inner = value.into_inner();

        let left = match inner.next() {
            Some(left) => Expr::try_from(left),
            None => Err(Error::map_span(value_span, "Expected a value expression")),
        }?;
        let members = inner
            .map(Expr::try_from)
            .collect::<Result<Vec<Expr>, Error>>()?;

        Ok(MemberExpr {
            left,
            members,
            lcol: value_lcol,
        })
    }
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

impl From<PrimaryExpr> for Expr {
    fn from(value: PrimaryExpr) -> Self {
        Expr::Primary(Box::new(value))
    }
}

impl From<UnaryExpr> for Expr {
    fn from(value: UnaryExpr) -> Self {
        if let Unary::Pass(PassUnary { expr, .. }) = value.unary {
            return expr;
        }
        Expr::Unary(Box::new(value))
    }
}

impl From<CallExpr> for Expr {
    fn from(value: CallExpr) -> Self {
        Expr::Call(Box::new(value))
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

impl From<BinaryExpr> for Expr {
    fn from(value: BinaryExpr) -> Self {
        if value.operator.is_none() && value.right.is_none() {
            return value.left;
        }
        Expr::Binary(Box::new(value))
    }
}

#[cfg(test)]
mod expr_tests {
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

    #[test]
    fn parses_call_expr() {
        let test_helper = ParserTestHelper::<CallExpr>::new(Rule::call_expr, "CallExpr");
        test_helper.assert_parse("test()");
        test_helper.assert_parse("5()");
        test_helper.assert_parse("\"Yo\"()");
        test_helper.assert_parse("false()");
    }

    #[test]
    fn parses_member_expr() {
        let test_helper = ParserTestHelper::<MemberExpr>::new(Rule::member_expr, "MemberExpr");
        test_helper.assert_parse("ident.fun().fun()");
        test_helper.assert_parse("ident.fun(arg1, arg2).fun(arg1, arg2)");
    }

    #[test]
    fn parses_binary_expr() {
        let test_helper = ParserTestHelper::<BinaryExpr>::new(Rule::logical_expr, "BinaryExpr");
        test_helper.assert_parse("5 + 2");
        test_helper.assert_parse("5/ 2");
        test_helper.assert_parse("5 *2");
        test_helper.assert_parse("5== 2");
    }
}
