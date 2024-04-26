use pest::iterators::Pair;

use crate::{
    ast::dfn::{mutator::MutatorDfn, object::ObjectDfn, path::PathDfn},
    parser::Rule,
};

use super::{Error, Expr};

#[derive(Debug, Clone)]
pub enum PrimaryExpr {
    Number(u32),
    Boolean(bool),
    Object(ObjectDfn),
    String(String),
    Grouping(Expr),
    RawString(String),
    Identifier(String),
    Mutator(MutatorDfn),
    Path(PathDfn),
    None,
}

impl TryFrom<Pair<'_, Rule>> for PrimaryExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let unary_expr_span = value.as_span();
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
                Some(interior) => Ok(PrimaryExpr::String(interior.as_str().to_string())),
                None => unreachable!(),
            },
            Rule::raw_string => match value.into_inner().next() {
                Some(interior) => Ok(PrimaryExpr::String(interior.as_str().to_string())),
                None => unreachable!(),
            },
            Rule::number => {
                let parsed_num = value.as_str().parse::<u32>().map_err(|_err| {
                    Error::map_span(
                        unary_expr_span,
                        format!("Unable to parse `{}` to number", value.as_str()),
                    )
                })?;
                return Ok(PrimaryExpr::Number(parsed_num));
            }
            Rule::grouping => match value.into_inner().next() {
                Some(expr) => Ok(PrimaryExpr::Grouping(Expr::try_from(expr)?)),
                None => unreachable!(),
            },
            Rule::none => Ok(PrimaryExpr::None),
            Rule::strict_ident => Ok(PrimaryExpr::Identifier(value.as_str().to_string())),
            Rule::raw_ident => match value.into_inner().next() {
                Some(interior) => Ok(PrimaryExpr::Identifier(interior.as_str().to_string())),
                None => unreachable!(),
            },
            Rule::path => Ok(PrimaryExpr::Path(PathDfn::try_from(value)?)),
            Rule::object => Ok(PrimaryExpr::Object(ObjectDfn::try_from(value)?)),
            Rule::mutator => Ok(PrimaryExpr::Mutator(MutatorDfn::try_from(value)?)),
            Rule::boolean => match value.as_str() {
                "true" => Ok(PrimaryExpr::Boolean(true)),
                "false" => Ok(PrimaryExpr::Boolean(false)),
                _ => Err(Error::map_span(unary_expr_span, "Invalid boolean value")),
            },
            _ => Err(Error::map_span(
                unary_expr_span,
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
