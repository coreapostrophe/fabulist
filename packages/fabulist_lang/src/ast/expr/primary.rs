use pest::iterators::Pair;

use crate::{ast::dfn::object::Object, parser::Rule};

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
            Rule::identifier => Ok(PrimaryExpr::Identifier(value.as_str().to_string())),
            Rule::object => Ok(PrimaryExpr::Object(Object::try_from(value)?)),
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
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_primary(source: &str) -> PrimaryExpr {
        let mut result =
            GrammarParser::parse(Rule::primary_expr, source).expect("Failed to parse string.");
        let primary = result.next().expect("Failed to parse primary expression");
        let primary_ast = PrimaryExpr::try_from(primary);
        assert!(primary_ast.is_ok());
        primary_ast.expect("Failed to turn pair to `Primary` struct")
    }

    #[test]
    fn parses_primaries() {
        parse_primary("\"string\"");
        parse_primary(r##"r"raw string""##);
        parse_primary("2");
        parse_primary("none");
        parse_primary("identifier");
        parse_primary(r#"{"string": "string", "number": 5}"#);
    }
}
