use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::expr::primitive::PrimitiveExpr, parser::Rule};

use super::Error;

#[derive(Debug, Clone)]
pub struct ParameterBodyDfn {
    pub lcol: LineColLocation,
    pub parameters: Option<Vec<PrimitiveExpr>>,
}

impl TryFrom<Pair<'_, Rule>> for ParameterBodyDfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let argument_body_dfn_lcol = LineColLocation::from(value.as_span());

        if let Some(parameters) = value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::parameters)
        {
            let param_expr = parameters
                .into_inner()
                .map(|pair| {
                    let pair_span = pair.as_span();
                    let primitive_expr = PrimitiveExpr::try_from(pair);
                    if let Ok(primitive_expr) = primitive_expr {
                        if let PrimitiveExpr::Identifier { .. } = primitive_expr {
                            return Ok(primitive_expr);
                        }
                    }
                    Err(Error::map_span(pair_span, "Expected identifier"))
                })
                .collect::<Result<Vec<PrimitiveExpr>, Error>>()?;
            Ok(ParameterBodyDfn {
                parameters: Some(param_expr),
                lcol: argument_body_dfn_lcol,
            })
        } else {
            Ok(ParameterBodyDfn {
                parameters: None,
                lcol: argument_body_dfn_lcol,
            })
        }
    }
}

#[cfg(test)]
mod argument_body_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    pub fn parses_parameter_body() {
        let test_helper =
            ParserTestHelper::<ParameterBodyDfn>::new(Rule::parameter_body, "ParameterBodyDfn");
        test_helper.assert_parse(r#"(param1, param2, param3)"#);
    }
}
