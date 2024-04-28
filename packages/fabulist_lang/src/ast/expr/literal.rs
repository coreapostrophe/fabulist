use pest::{error::LineColLocation, iterators::Pair};

use crate::parser::Rule;

use super::Error;

#[derive(Debug, Clone)]
pub enum LiteralExpr {
    Number {
        value: u32,
        lcol: LineColLocation,
    },
    Boolean {
        value: bool,
        lcol: LineColLocation,
    },
    String {
        value: String,
        lcol: LineColLocation,
    },
    None {
        lcol: LineColLocation,
    },
}

impl TryFrom<Pair<'_, Rule>> for LiteralExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let literal_expr_span = value.as_span();
        let literal_expr_lcol = LineColLocation::from(literal_expr_span);

        match value.as_rule() {
            Rule::literal_expr => match value.into_inner().next() {
                Some(inner) => Ok(LiteralExpr::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::string => match value.into_inner().next() {
                Some(interior) => Ok(LiteralExpr::String {
                    value: interior.as_str().to_string(),
                    lcol: literal_expr_lcol,
                }),
                None => unreachable!(),
            },
            Rule::raw_string => match value.into_inner().next() {
                Some(interior) => Ok(LiteralExpr::String {
                    value: interior.as_str().to_string(),
                    lcol: literal_expr_lcol,
                }),
                None => unreachable!(),
            },
            Rule::number => {
                let parsed_number = value.as_str().parse::<u32>().map_err(|_| {
                    Error::map_span(
                        literal_expr_span,
                        format!("Unable to parse `{}` to number", value.as_str()),
                    )
                })?;
                Ok(LiteralExpr::Number {
                    value: parsed_number,
                    lcol: literal_expr_lcol,
                })
            }
            Rule::none => Ok(LiteralExpr::None {
                lcol: literal_expr_lcol,
            }),
            Rule::boolean => match value.as_str() {
                "true" => Ok(LiteralExpr::Boolean {
                    value: true,
                    lcol: literal_expr_lcol,
                }),
                "false" => Ok(LiteralExpr::Boolean {
                    value: false,
                    lcol: literal_expr_lcol,
                }),
                _ => unreachable!(),
            },
            _ => Err(Error::map_span(
                literal_expr_span,
                "Invalid literal expression",
            )),
        }
    }
}
