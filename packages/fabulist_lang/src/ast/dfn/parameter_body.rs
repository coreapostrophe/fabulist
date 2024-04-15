use pest::iterators::Pair;

use crate::{ast::expr::primary::PrimaryExpr, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct ParameterBody(pub Option<Vec<PrimaryExpr>>);

impl TryFrom<Pair<'_, Rule>> for ParameterBody {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        if let Some(parameters) = value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::parameters)
        {
            let param_ident = parameters
                .into_inner()
                .map(|pair| {
                    let primary = PrimaryExpr::try_from(pair)?;
                    match primary {
                        PrimaryExpr::Identifier(_) => Ok(primary),
                        _ => Err(Error::InvalidRule(value_rule)),
                    }
                })
                .collect::<Result<Vec<PrimaryExpr>, Error>>()?;
            Ok(ParameterBody(Some(param_ident)))
        } else {
            Ok(ParameterBody(None))
        }
    }
}

#[cfg(test)]
mod parameter_body_tests {
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_parameter_body(source: &str) -> ParameterBody {
        let mut result =
            GrammarParser::parse(Rule::parameter_body, source).expect("Failed to parse string.");
        let parameter_body = result.next().expect("Failed to parse parameter body");
        let parameter_body_ast = ParameterBody::try_from(parameter_body);
        assert!(parameter_body_ast.is_ok());
        parameter_body_ast.expect("Failed to turn pair to `ParameterBody` struct")
    }

    #[test]
    fn parses_parameter_body() {
        parse_parameter_body("(test, test2, test3)");
    }
}
