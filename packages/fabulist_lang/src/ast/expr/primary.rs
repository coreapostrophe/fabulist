use pest::iterators::Pair;

use crate::{
    ast::dfn::{mutator::Mutator, object::Object},
    parser::Rule,
};

use super::{Error, Expr};

#[derive(Debug)]
pub enum PrimaryExpr {
    Number(u32),
    Boolean(bool),
    Object(Object),
    String(String),
    Grouping(Expr),
    RawString(String),
    Identifier(String),
    Mutator(Mutator),
    None,
}

impl TryFrom<Pair<'_, Rule>> for PrimaryExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        match value_rule {
            Rule::primary_expr => match value.into_inner().next() {
                Some(inner) => Ok(PrimaryExpr::try_from(inner)?),
                None => Err(Error::InvalidRule(value_rule)),
            },
            Rule::identifier => match value.into_inner().next() {
                Some(inner) => Ok(PrimaryExpr::try_from(inner)?),
                None => Err(Error::InvalidRule(value_rule)),
            },
            Rule::string => match value.into_inner().next() {
                Some(interior) => Ok(PrimaryExpr::String(interior.as_str().to_string())),
                None => Err(Error::InvalidRule(value_rule)),
            },
            Rule::raw_string => match value.into_inner().next() {
                Some(interior) => Ok(PrimaryExpr::String(interior.as_str().to_string())),
                None => Err(Error::InvalidRule(value_rule)),
            },
            Rule::number => {
                let parsed_num = value
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_err| Error::InvalidNumber(value.as_str().to_string()))?;
                return Ok(PrimaryExpr::Number(parsed_num));
            }
            Rule::grouping => match value.into_inner().next() {
                Some(expr) => Ok(PrimaryExpr::Grouping(Expr::try_from(expr)?)),
                None => Err(Error::InvalidRule(value_rule)),
            },
            Rule::none => Ok(PrimaryExpr::None),
            Rule::strict_ident => Ok(PrimaryExpr::Identifier(value.as_str().to_string())),
            Rule::raw_ident => match value.into_inner().next() {
                Some(interior) => Ok(PrimaryExpr::Identifier(interior.as_str().to_string())),
                None => Err(Error::InvalidRule(value_rule)),
            },
            Rule::object => Ok(PrimaryExpr::Object(Object::try_from(value)?)),
            Rule::mutator => Ok(PrimaryExpr::Mutator(Mutator::try_from(value)?)),
            Rule::boolean => match value.as_str() {
                "true" => Ok(PrimaryExpr::Boolean(true)),
                "false" => Ok(PrimaryExpr::Boolean(false)),
                _ => Err(Error::InvalidRule(value_rule)),
            },
            _ => Err(Error::InvalidRule(value_rule)),
        }
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
        test_helper.assert_parse(r#"{"string": "string", "number": 5}"#);
    }
}
