use fabulist_derive::SyntaxTree;
use pest::{error::LineColLocation, iterators::Pair};

use crate::parser::Rule;

use super::Error;

#[derive(SyntaxTree, Debug, Clone)]
pub enum Literal {
    #[production(value: f32)]
    Number(NumberLiteral),
    #[production(value: bool)]
    Boolean(BooleanLiteral),
    #[production(value: String)]
    String(StringLiteral),
    #[production]
    None(NoneLiteral),
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

#[cfg(test)]
mod literal_expr_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_literal_expr() {
        let test_helper = ParserTestHelper::<Literal>::new(Rule::literal_expr, "LiteralExpr");
        test_helper.assert_parse("\"string\"");
        test_helper.assert_parse("r#\"raw string\"#");
        test_helper.assert_parse("5");
        test_helper.assert_parse("5.52252");
        test_helper.assert_parse("none");
        test_helper.assert_parse("true");
        test_helper.assert_parse("false");
    }
}
