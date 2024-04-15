use pest::iterators::Pair;

use crate::{ast::expr::Expr, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct ArgumentBody(pub Option<Vec<Expr>>);

impl TryFrom<Pair<'_, Rule>> for ArgumentBody {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        if let Some(arguments) = value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::arguments)
        {
            let arg_expr = arguments
                .into_inner()
                .map(|pair| Expr::try_from(pair))
                .collect::<Result<Vec<Expr>, Error>>()?;
            Ok(ArgumentBody(Some(arg_expr)))
        } else {
            Ok(ArgumentBody(None))
        }
    }
}

#[cfg(test)]
mod argument_body_tests {
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_argument_body(source: &str) -> ArgumentBody {
        let mut result =
            GrammarParser::parse(Rule::argument_body, source).expect("Failed to parse string.");
        let argument_body = result.next().expect("Failed to parse argument body");
        let argument_body_ast = ArgumentBody::try_from(argument_body);
        assert!(argument_body_ast.is_ok());
        argument_body_ast.expect("Failed to turn pair to `ArgumentBody` struct")
    }

    #[test]
    pub fn parses_argument_body() {
        parse_argument_body(r#"("string", 5, true)"#);
    }
}
