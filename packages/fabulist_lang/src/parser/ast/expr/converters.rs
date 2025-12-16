//! Converters from pest parse pairs into expression AST nodes.
use pest::iterators::Pair;

use crate::parser::{
    ast::{
        dfn::models::{ArgumentBodyDfn, ObjectDfn, ParameterBodyDfn},
        expr::models::AssignmentExpr,
        stmt::models::BlockStmt,
    },
    error::{ExtractSpanSlice, ParserError},
    Rule,
};

use super::models::{
    BinaryExpr, BinaryOperator, BooleanLiteral, CallExpr, ContextPrimitive, Expr,
    GroupingPrimitive, IdentifierPrimitive, LambdaPrimitive, Literal, LiteralPrimary, MemberExpr,
    NoneLiteral, NumberLiteral, ObjectPrimitive, PassUnary, PathPrimitive, Primary, PrimaryExpr,
    Primitive, PrimitivePrimary, StandardUnary, StringLiteral, Unary, UnaryExpr, UnaryOperator,
};

impl TryFrom<Pair<'_, Rule>> for Unary {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        let mut inner = value.into_inner();

        if let Some(member) = inner
            .clone()
            .find(|pair| pair.as_rule() == Rule::member_expr)
        {
            Ok(Unary::Pass(PassUnary {
                span_slice: value_span_slice,
                expr: Expr::try_from(member)?,
            }))
        } else {
            let operator = match inner.find(|pair| pair.as_node_tag() == Some("operator")) {
                Some(operator) => match operator.as_str() {
                    "-" => Ok(UnaryOperator::Negation),
                    "!" => Ok(UnaryOperator::Not),
                    _ => Err(ParserError::ExpectedSymbol {
                        expected: "unary operator".to_string(),
                        span_slice: operator.extract_span_slice(),
                    }),
                },
                None => Err(ParserError::ExpectedSymbol {
                    expected: "unary operator".to_string(),
                    span_slice: value_span_slice.clone(),
                }),
            }?;
            let right = match inner.find(|pair| pair.as_node_tag() == Some("right")) {
                Some(right) => Ok(Expr::try_from(right)?),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "expression".to_string(),
                    span_slice: value_span_slice.clone(),
                }),
            }?;

            Ok(Unary::Standard(StandardUnary {
                span_slice: value_span_slice,
                operator,
                right,
            }))
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Expr {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        match value.as_rule() {
            Rule::expression => match value.into_inner().next() {
                Some(inner) => Ok(Expr::try_from(inner)?),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "expression".to_string(),
                    span_slice: value_span_slice,
                }),
            },

            Rule::unary_expr => Ok(UnaryExpr::try_from(value)?.into()),
            Rule::call_expr => Ok(CallExpr::try_from(value)?.into()),
            Rule::member_expr => Ok(MemberExpr::try_from(value)?.into()),
            Rule::assignment_expr => Ok(AssignmentExpr::try_from(value)?.into()),

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

            _ => Err(ParserError::InvalidExpression(value_span_slice)),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for PrimaryExpr {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(PrimaryExpr {
            span_slice: value.extract_span_slice(),
            primary: Primary::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for UnaryExpr {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(UnaryExpr {
            span_slice: value.extract_span_slice(),
            unary: Unary::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for CallExpr {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let callee = match inner.find(|pair| pair.as_node_tag() == Some("callee")) {
            Some(callee) => Expr::try_from(callee),
            None => Err(ParserError::ExpectedSymbol {
                expected: "callee expression".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;
        let argument_body = match inner.find(|pair| pair.as_rule() == Rule::argument_body) {
            Some(argument_body) => Some(ArgumentBodyDfn::try_from(argument_body)?),
            None => None,
        };

        Ok(CallExpr {
            span_slice: value_span_slice,
            callee,
            argument_body,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for MemberExpr {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let left = match inner.next() {
            Some(left) => Expr::try_from(left),
            None => Err(ParserError::ExpectedSymbol {
                expected: "value expression".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;
        let members = inner
            .map(Expr::try_from)
            .collect::<Result<Vec<Expr>, ParserError>>()?;

        Ok(MemberExpr {
            span_slice: value_span_slice,
            left,
            members,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for BinaryExpr {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let left = match inner.find(|pair| pair.as_node_tag() == Some("left")) {
            Some(left) => Expr::try_from(left),
            None => Err(ParserError::ExpectedSymbol {
                expected: "value expression".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        let operator = match inner.find(|pair| pair.as_node_tag() == Some("operator")) {
            Some(operator) => Some(match operator.as_str() {
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
                _ => Err(ParserError::InvalidBinaryOperator(
                    operator.extract_span_slice(),
                )),
            }?),
            None => None,
        };

        let right = match inner.find(|pair| pair.as_node_tag() == Some("right")) {
            Some(right) => Some(Expr::try_from(right)?),
            None => None,
        };

        Ok(BinaryExpr {
            span_slice: value_span_slice,
            left,
            operator,
            right,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for AssignmentExpr {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let left = match inner.find(|pair| pair.as_node_tag() == Some("left")) {
            Some(left) => Expr::try_from(left),
            None => Err(ParserError::ExpectedSymbol {
                expected: "target expression".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        let right = match inner.find(|pair| pair.as_node_tag() == Some("right")) {
            Some(right) => Some(Expr::try_from(right)?),
            None => None,
        };

        Ok(AssignmentExpr {
            span_slice: value_span_slice,
            left,
            right,
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

impl From<AssignmentExpr> for Expr {
    fn from(value: AssignmentExpr) -> Self {
        if value.right.is_none() {
            return value.left;
        }
        Expr::Assignment(Box::new(value))
    }
}

impl TryFrom<Pair<'_, Rule>> for Primary {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        match value.as_rule() {
            Rule::primary_expr => match value.into_inner().next() {
                Some(inner) => Ok(Primary::try_from(inner)?),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "primary expression".to_string(),
                    span_slice: value_span_slice.clone(),
                }),
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

            _ => Err(ParserError::InvalidPrimaryExpression(value_span_slice)),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for LiteralPrimary {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(LiteralPrimary {
            span_slice: value.extract_span_slice(),
            literal: Literal::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for PrimitivePrimary {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(PrimitivePrimary {
            span_slice: value.extract_span_slice(),
            primitive: Primitive::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for Literal {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        match value.as_rule() {
            Rule::literal_expr => match value.into_inner().next() {
                Some(inner) => Ok(Literal::try_from(inner)?),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "literal expression".to_string(),
                    span_slice: value_span_slice,
                }),
            },
            Rule::string => match value.into_inner().next() {
                Some(inner) => Ok(Literal::try_from(inner)?),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "string".to_string(),
                    span_slice: value_span_slice,
                }),
            },
            Rule::raw_string => match value.into_inner().next() {
                Some(inner) => Ok(Literal::try_from(inner)?),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "raw string".to_string(),
                    span_slice: value_span_slice,
                }),
            },
            Rule::string_interior => Ok(Literal::String(StringLiteral {
                span_slice: value_span_slice,
                value: value.as_str().to_string(),
            })),
            Rule::raw_string_interior => Ok(Literal::String(StringLiteral {
                span_slice: value_span_slice,
                value: value.as_str().to_string(),
            })),
            Rule::number => {
                let parsed_number = value
                    .as_str()
                    .parse::<f32>()
                    .map_err(|_| ParserError::UnableToCastNumber(value_span_slice.clone()))?;

                Ok(Literal::Number(NumberLiteral {
                    span_slice: value_span_slice,
                    value: parsed_number,
                }))
            }
            Rule::boolean => match value.as_str() {
                "true" => Ok(Literal::Boolean(BooleanLiteral {
                    span_slice: value_span_slice,
                    value: true,
                })),
                "false" => Ok(Literal::Boolean(BooleanLiteral {
                    span_slice: value_span_slice,
                    value: false,
                })),
                _ => Err(ParserError::InvalidBooleanLiteral(value_span_slice.clone())),
            },
            Rule::none => Ok(Literal::None(NoneLiteral {
                span_slice: value_span_slice,
            })),
            _ => Err(ParserError::InvalidLiteral(value_span_slice)),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Primitive {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        match value.as_rule() {
            Rule::primitive_expr => match value.into_inner().next() {
                Some(inner) => Ok(Primitive::try_from(inner)?),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "primitive".to_string(),
                    span_slice: value_span_slice,
                }),
            },
            Rule::grouping => Ok(Primitive::Grouping(GroupingPrimitive::try_from(value)?)),
            Rule::identifier | Rule::strict_ident | Rule::raw_ident => {
                Ok(Primitive::Identifier(IdentifierPrimitive::try_from(value)?))
            }
            Rule::path => Ok(Primitive::Path(PathPrimitive::try_from(value)?)),
            Rule::object => Ok(Primitive::Object(ObjectPrimitive {
                span_slice: value_span_slice,
                object: ObjectDfn::try_from(value)?,
            })),
            Rule::lambda => Ok(Primitive::Lambda(LambdaPrimitive::try_from(value)?)),
            Rule::context => Ok(Primitive::Context(ContextPrimitive {
                span_slice: value_span_slice,
            })),
            _ => Err(ParserError::InvalidPrimitive(value_span_slice)),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for IdentifierPrimitive {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

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
                span_slice: value_span_slice.clone(),
                name: value.as_str().to_string(),
            }),
            _ => Err(ParserError::InvalidIdentifierPrimitive(value_span_slice)),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for GroupingPrimitive {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        let expr = match value.into_inner().next() {
            Some(expr) => Ok(Expr::try_from(expr)?),
            None => Err(ParserError::ExpectedSymbol {
                expected: "expression".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        Ok(GroupingPrimitive {
            span_slice: value_span_slice,
            expr,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for PathPrimitive {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        let identifiers = value
            .into_inner()
            .map(IdentifierPrimitive::try_from)
            .collect::<Result<Vec<IdentifierPrimitive>, ParserError>>()?;

        Ok(PathPrimitive {
            span_slice: value_span_slice,
            identifiers,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for LambdaPrimitive {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let parameters = match inner.find(|pair| pair.as_rule() == Rule::parameter_body) {
            Some(parameter_body_dfn) => ParameterBodyDfn::try_from(parameter_body_dfn),
            None => Err(ParserError::ExpectedSymbol {
                expected: "parameter body".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        let block_stmt = match inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(ParserError::ExpectedSymbol {
                expected: "block statement".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        Ok(LambdaPrimitive {
            span_slice: value_span_slice,
            block_stmt,
            parameters,
        })
    }
}

#[cfg(test)]
mod expr_converters_tests {
    use crate::{parser::ast::AstTestHelper, parser::Rule};

    use super::*;

    #[test]
    fn parses_unary_expr() {
        let test_helper = AstTestHelper::<UnaryExpr>::new(Rule::unary_expr, "UnaryExpr");
        test_helper.assert_parse("!5");
        test_helper.assert_parse("!(true)");
        test_helper.assert_parse("!!!ident");
        test_helper.assert_parse("-\"num\"");
    }

    #[test]
    fn parses_call_expr() {
        let test_helper = AstTestHelper::<CallExpr>::new(Rule::call_expr, "CallExpr");
        test_helper.assert_parse("test()");
        test_helper.assert_parse("5()");
        test_helper.assert_parse("\"Yo\"()");
        test_helper.assert_parse("false()");
    }

    #[test]
    fn parses_member_expr() {
        let test_helper = AstTestHelper::<MemberExpr>::new(Rule::member_expr, "MemberExpr");
        test_helper.assert_parse("ident.fun().fun()");
        test_helper.assert_parse("ident.fun(arg1, arg2).fun(arg1, arg2)");
    }

    #[test]
    fn parses_binary_expr() {
        let test_helper = AstTestHelper::<BinaryExpr>::new(Rule::logical_expr, "BinaryExpr");
        test_helper.assert_parse("5 + 2");
        test_helper.assert_parse("5/ 2");
        test_helper.assert_parse("5 *2");
        test_helper.assert_parse("5== 2");
    }

    #[test]
    fn parses_assignment_expr() {
        let test_helper =
            AstTestHelper::<AssignmentExpr>::new(Rule::assignment_expr, "AssignmentExpr");
        test_helper.assert_parse("a = 5");
        test_helper.assert_parse("b = a + 2");
    }

    #[test]
    fn parses_primaries() {
        let test_helper = AstTestHelper::<Primary>::new(Rule::primary_expr, "PrimaryExpr");
        test_helper.assert_parse("\"string\"");
        test_helper.assert_parse(r##"r"raw string""##);
        test_helper.assert_parse("2");
        test_helper.assert_parse("2.5");
        test_helper.assert_parse("none");
        test_helper.assert_parse("identifier");
        test_helper.assert_parse("r#none");
        test_helper.assert_parse("path::path_2::path_3");
        test_helper.assert_parse(r#"{"string": "string", "number": 5}"#);
    }

    #[test]
    fn parses_literal_expr() {
        let test_helper = AstTestHelper::<Literal>::new(Rule::literal_expr, "LiteralExpr");
        test_helper.assert_parse("\"string\"");
        test_helper.assert_parse("r#\"raw string\"#");
        test_helper.assert_parse("5");
        test_helper.assert_parse("5.52252");
        test_helper.assert_parse("none");
        test_helper.assert_parse("true");
        test_helper.assert_parse("false");
    }

    #[test]
    fn parses_primitive_expr() {
        let test_helper = AstTestHelper::<Primitive>::new(Rule::primitive_expr, "PrimitiveExpr");
        test_helper.assert_parse("ident");
        test_helper.assert_parse("r#module");
        test_helper.assert_parse("(ident)");
        test_helper.assert_parse("path::path2::path3");
        test_helper.assert_parse("{ \"key\": 5 }");
        test_helper.assert_parse("() => { goto module_1::part_1; }");
        test_helper.assert_parse("(param1, param2) => { let a = 5; }");
        test_helper.assert_parse("context");
    }
}
