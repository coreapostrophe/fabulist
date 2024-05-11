use pest::{error::LineColLocation, iterators::Pair};

use crate::{
    ast::{
        dfn::{ArgumentBodyDfn, ObjectDfn, ParameterBodyDfn},
        stmt::BlockStmt,
    },
    error::Error,
    parser::Rule,
};

use super::models::{
    BinaryExpr, BinaryOperator, BooleanLiteral, CallExpr, ContextPrimitive, Expr,
    GroupingPrimitive, IdentifierPrimitive, LambdaPrimitive, Literal, LiteralPrimary, MemberExpr,
    NoneLiteral, NumberLiteral, ObjectPrimitive, PassUnary, PathPrimitive, Primary, PrimaryExpr,
    Primitive, PrimitivePrimary, StandardUnary, StringLiteral, Unary, UnaryExpr, UnaryOperator,
};

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
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let mut inner = value.into_inner();

        let left = match inner.find(|pair| pair.as_node_tag() == Some("left")) {
            Some(left) => Expr::try_from(left),
            None => Err(Error::map_span(value_span, "Expected a value expression")),
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
            lcol: value_lcol,
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

impl TryFrom<Pair<'_, Rule>> for Primary {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        match value.as_rule() {
            Rule::primary_expr => match value.into_inner().next() {
                Some(inner) => Ok(Primary::try_from(inner)?),
                None => Err(Error::map_span(
                    value_span,
                    "Unable to parse token tree interior",
                )),
            },
            Rule::primitive_expr
            | Rule::identifier
            | Rule::strict_ident
            | Rule::raw_ident
            | Rule::path
            | Rule::object
            | Rule::lambda
            | Rule::grouping => Ok(Primary::Primitive(PrimitivePrimary::try_from(value)?)),
            Rule::literal_expr
            | Rule::string
            | Rule::raw_string
            | Rule::number
            | Rule::none
            | Rule::boolean => Ok(Primary::Literal(LiteralPrimary::try_from(value)?)),
            _ => Err(Error::map_span(value_span, "Invalid primary expression")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for LiteralPrimary {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());
        Ok(LiteralPrimary {
            lcol: value_lcol,
            literal: Literal::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for PrimitivePrimary {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());
        Ok(PrimitivePrimary {
            lcol: value_lcol,
            primitive: Primitive::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for Literal {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);

        match value.as_rule() {
            Rule::literal_expr => match value.into_inner().next() {
                Some(inner) => Ok(Literal::try_from(inner)?),
                None => Err(Error::map_span(
                    value_span,
                    "Unable to parse token tree interior",
                )),
            },
            Rule::string => match value.into_inner().next() {
                Some(inner) => Ok(Literal::try_from(inner)?),
                None => Err(Error::map_span(
                    value_span,
                    "Unable to parse token tree interior",
                )),
            },
            Rule::raw_string => match value.into_inner().next() {
                Some(inner) => Ok(Literal::try_from(inner)?),
                None => Err(Error::map_span(
                    value_span,
                    "Unable to parse token tree interior",
                )),
            },
            Rule::string_interior => Ok(Literal::String(StringLiteral {
                lcol: value_lcol,
                value: value.as_str().to_string(),
            })),
            Rule::raw_string_interior => Ok(Literal::String(StringLiteral {
                lcol: value_lcol,
                value: value.as_str().to_string(),
            })),
            Rule::number => {
                let parsed_number = value.as_str().parse::<f32>().map_err(|_| {
                    Error::map_span(
                        value_span,
                        format!("Unable to parse `{}` to number", value.as_str()),
                    )
                })?;
                Ok(Literal::Number(NumberLiteral {
                    lcol: value_lcol,
                    value: parsed_number,
                }))
            }
            Rule::boolean => match value.as_str() {
                "true" => Ok(Literal::Boolean(BooleanLiteral {
                    lcol: value_lcol,
                    value: true,
                })),
                "false" => Ok(Literal::Boolean(BooleanLiteral {
                    lcol: value_lcol,
                    value: false,
                })),
                _ => Err(Error::map_span(value_span, "Invalid boolean value")),
            },
            Rule::none => Ok(Literal::None(NoneLiteral { lcol: value_lcol })),
            _ => Err(Error::map_span(value_span, "Invalid literal expression")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Primitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);

        match value.as_rule() {
            Rule::primitive_expr => match value.into_inner().next() {
                Some(inner) => Ok(Primitive::try_from(inner)?),
                None => Err(Error::map_span(value_span, "Invalid primitive expression")),
            },
            Rule::grouping => Ok(Primitive::Grouping(GroupingPrimitive::try_from(value)?)),
            Rule::identifier | Rule::strict_ident | Rule::raw_ident => {
                Ok(Primitive::Identifier(IdentifierPrimitive::try_from(value)?))
            }
            Rule::path => Ok(Primitive::Path(PathPrimitive::try_from(value)?)),
            Rule::object => Ok(Primitive::Object(ObjectPrimitive {
                lcol: value_lcol,
                object: ObjectDfn::try_from(value)?,
            })),
            Rule::lambda => Ok(Primitive::Lambda(LambdaPrimitive::try_from(value)?)),
            Rule::context => Ok(Primitive::Context(ContextPrimitive { lcol: value_lcol })),
            _ => Err(Error::map_span(value_span, "Invalid primitive expression")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for IdentifierPrimitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);

        match value.as_rule() {
            Rule::identifier => match value.into_inner().next() {
                Some(inner) => Ok(IdentifierPrimitive::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::strict_ident => match value.into_inner().next() {
                Some(inner) => Ok(IdentifierPrimitive::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::raw_ident => match value.into_inner().next() {
                Some(inner) => Ok(IdentifierPrimitive::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::ident_interior => Ok(IdentifierPrimitive {
                lcol: value_lcol,
                name: value.as_str().to_string(),
            }),
            _ => Err(Error::map_span(value_span, "Invalid primitive")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for GroupingPrimitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);

        let expr = match value.into_inner().next() {
            Some(expr) => Ok(Expr::try_from(expr)?),
            None => Err(Error::map_span(value_span, "Expected expression")),
        }?;

        Ok(GroupingPrimitive {
            lcol: value_lcol,
            expr,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for PathPrimitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());
        let identifiers = value
            .into_inner()
            .map(IdentifierPrimitive::try_from)
            .collect::<Result<Vec<IdentifierPrimitive>, Error>>()?;

        Ok(PathPrimitive {
            lcol: value_lcol,
            identifiers,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for LambdaPrimitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let mut inner = value.into_inner();

        let parameters = match inner.find(|pair| pair.as_rule() == Rule::parameter_body) {
            Some(parameter_body_dfn) => ParameterBodyDfn::try_from(parameter_body_dfn),
            None => Err(Error::map_span(value_span, "Expected parameter body")),
        }?;

        let block_stmt = match inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(Error::map_span(value_span, "Expected a block statement")),
        }?;

        Ok(LambdaPrimitive {
            lcol: value_lcol,
            block_stmt,
            parameters,
        })
    }
}
