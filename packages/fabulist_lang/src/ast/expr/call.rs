use pest::iterators::Pair;

use crate::{ast::dfn::argument_body::ArgumentBody, parser::Rule};

use super::{Error, Expr};

#[derive(Debug)]
pub struct CallExpr {
    pub callee: Expr,
    pub argument_body: Option<ArgumentBody>,
}

impl TryFrom<Pair<'_, Rule>> for CallExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let mut inner = value.into_inner();

        let callee = match inner.find_first_tagged("callee") {
            Some(callee) => Ok(Expr::try_from(callee)?),
            None => Err(Error::InvalidRule(value_rule)),
        }?;
        let argument_body = match inner.find(|pair| pair.as_rule() == Rule::argument_body) {
            Some(argument_body) => Some(ArgumentBody::try_from(argument_body)?),
            None => None,
        };

        Ok(CallExpr {
            callee,
            argument_body,
        })
    }
}

#[cfg(test)]
mod call_expr_tests {
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_call_expr(source: &str) -> CallExpr {
        let mut result = GrammarParser::parse(Rule::call, source).expect("Failed to parse string.");
        let call = result.next().expect("Failed to parse call expression");
        let call_ast = CallExpr::try_from(call);
        assert!(call_ast.is_ok());
        call_ast.expect("Failed to turn pair to `CallExpr` struct")
    }

    #[test]
    fn parses_call_expr() {
        parse_call_expr("test()");
        parse_call_expr("5()");
        parse_call_expr("\"Yo\"()");
        parse_call_expr("false()");
    }
}
