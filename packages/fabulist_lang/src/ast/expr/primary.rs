use pest::{error::LineColLocation, iterators::Pair};

use crate::parser::Rule;

use super::{literal::LiteralExpr, primitive::PrimitiveExpr, Error, Expr};

#[derive(Debug, Clone)]
pub enum PrimaryExpr {
    Literal {
        value: LiteralExpr,
        lcol: LineColLocation,
    },
    Primitive {
        value: PrimitiveExpr,
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
            Rule::primitive_expr
            | Rule::identifier
            | Rule::strict_ident
            | Rule::raw_ident
            | Rule::path
            | Rule::object
            | Rule::mutator
            | Rule::grouping => match value.into_inner().next() {
                Some(inner) => Ok(PrimaryExpr::Primitive {
                    value: PrimitiveExpr::try_from(inner)?,
                    lcol: primary_expr_lcol,
                }),
                None => unreachable!(),
            },
            Rule::literal_expr
            | Rule::string
            | Rule::raw_string
            | Rule::number
            | Rule::none
            | Rule::boolean => match value.into_inner().next() {
                Some(inner) => Ok(PrimaryExpr::Literal {
                    value: LiteralExpr::try_from(inner)?,
                    lcol: primary_expr_lcol,
                }),
                None => unreachable!(),
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
