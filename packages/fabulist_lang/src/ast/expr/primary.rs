use fabulist_derive::SyntaxTree;
use pest::{error::LineColLocation, iterators::Pair};

use crate::{error::Error, parser::Rule};

use super::{literal::Literal, primitive::Primitive};

#[derive(SyntaxTree, Debug, Clone)]
pub enum Primary {
    #[production(literal: Literal)]
    Literal(LiteralPrimary),
    #[production(primitive: Primitive)]
    Primitive(PrimitivePrimary),
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

#[cfg(test)]
mod primary_expr_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_primaries() {
        let test_helper = ParserTestHelper::<Primary>::new(Rule::primary_expr, "PrimaryExpr");
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
}
