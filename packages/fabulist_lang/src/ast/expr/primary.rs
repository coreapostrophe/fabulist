use pest::{error::LineColLocation, iterators::Pair};

use crate::{
    ast::dfn::{mutator::MutatorDfn, object::ObjectDfn, path::PathDfn},
    parser::Rule,
};

use super::{Error, Expr};

#[derive(Debug, Clone)]
pub enum PrimaryExpr {
    Number {
        value: u32,
        lcol: LineColLocation,
    },
    Boolean {
        value: bool,
        lcol: LineColLocation,
    },
    Object {
        value: ObjectDfn,
        lcol: LineColLocation,
    },
    String {
        value: String,
        lcol: LineColLocation,
    },
    Grouping {
        value: Expr,
        lcol: LineColLocation,
    },
    Identifier {
        value: String,
        lcol: LineColLocation,
    },
    Mutator {
        value: MutatorDfn,
        lcol: LineColLocation,
    },
    Path {
        value: PathDfn,
        lcol: LineColLocation,
    },
    None {
        lcol: LineColLocation,
    },
}

impl TryFrom<Pair<'_, Rule>> for PrimaryExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let primary_expr_span = value.as_span();
        let primary_expr_lcol = LineColLocation::from(primary_expr_span);

        match value.as_rule() {
            Rule::primary_expr => match value.into_inner().next() {
                Some(inner) => Ok(PrimaryExpr::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::identifier => match value.into_inner().next() {
                Some(inner) => Ok(PrimaryExpr::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::string => match value.into_inner().next() {
                Some(interior) => Ok(PrimaryExpr::String {
                    value: interior.as_str().to_string(),
                    lcol: primary_expr_lcol,
                }),
                None => unreachable!(),
            },
            Rule::raw_string => match value.into_inner().next() {
                Some(interior) => Ok(PrimaryExpr::String {
                    value: interior.as_str().to_string(),
                    lcol: primary_expr_lcol,
                }),
                None => unreachable!(),
            },
            Rule::number => {
                let parsed_number = value.as_str().parse::<u32>().map_err(|_| {
                    Error::map_span(
                        primary_expr_span,
                        format!("Unable to parse `{}` to number", value.as_str()),
                    )
                })?;
                Ok(PrimaryExpr::Number {
                    value: parsed_number,
                    lcol: primary_expr_lcol,
                })
            }
            Rule::grouping => match value.into_inner().next() {
                Some(expr) => Ok(PrimaryExpr::Grouping {
                    value: Expr::try_from(expr)?,
                    lcol: primary_expr_lcol,
                }),
                None => unreachable!(),
            },
            Rule::none => Ok(PrimaryExpr::None {
                lcol: primary_expr_lcol,
            }),
            Rule::strict_ident => Ok(PrimaryExpr::Identifier {
                value: value.as_str().to_string(),
                lcol: primary_expr_lcol,
            }),
            Rule::raw_ident => match value.into_inner().next() {
                Some(interior) => Ok(PrimaryExpr::Identifier {
                    value: interior.as_str().to_string(),
                    lcol: primary_expr_lcol,
                }),
                None => unreachable!(),
            },
            Rule::path => Ok(PrimaryExpr::Path {
                value: PathDfn::try_from(value)?,
                lcol: primary_expr_lcol,
            }),
            Rule::object => Ok(PrimaryExpr::Object {
                value: ObjectDfn::try_from(value)?,
                lcol: primary_expr_lcol,
            }),
            Rule::mutator => Ok(PrimaryExpr::Mutator {
                value: MutatorDfn::try_from(value)?,
                lcol: primary_expr_lcol,
            }),
            Rule::boolean => match value.as_str() {
                "true" => Ok(PrimaryExpr::Boolean {
                    value: true,
                    lcol: primary_expr_lcol,
                }),
                "false" => Ok(PrimaryExpr::Boolean {
                    value: false,
                    lcol: primary_expr_lcol,
                }),
                _ => unreachable!(),
            },
            _ => Err(Error::map_span(
                primary_expr_span,
                "Invalid primary expression",
            )),
        }
    }
}

impl From<PrimaryExpr> for Expr {
    fn from(value: PrimaryExpr) -> Self {
        Expr::Primary(Box::new(value))
    }
}

#[cfg(test)]
mod primary_expr_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_primaries() {
        let test_helper = ParserTestHelper::<PrimaryExpr>::new(Rule::primary_expr, "PrimaryExpr");
        test_helper.assert_parse("\"string\"");
        test_helper.assert_parse(r##"r"raw string""##);
        test_helper.assert_parse("2");
        test_helper.assert_parse("none");
        test_helper.assert_parse("identifier");
        test_helper.assert_parse("r#none");
        test_helper.assert_parse("path::path_2::path_3");
        test_helper.assert_parse(r#"{"string": "string", "number": 5}"#);
    }
}
