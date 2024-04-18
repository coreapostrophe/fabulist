use pest::iterators::Pair;

use crate::{ast::dfn::object::Object, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct QuoteElem {
    pub text: String,
    pub properties: Option<Object>,
}

impl TryFrom<Pair<'_, Rule>> for QuoteElem {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let mut inner = value.into_inner();

        let text = match inner.find_first_tagged("text") {
            Some(text) => Ok(match text.into_inner().next() {
                Some(text) => Ok(text.as_str().to_string()),
                None => Err(Error::InvalidRule(value_rule)),
            }?),
            None => Err(Error::InvalidRule(value_rule)),
        }?;

        let properties = match inner.find(|pair| pair.as_rule() == Rule::object) {
            Some(object) => Some(Object::try_from(object)?),
            None => None,
        };

        Ok(QuoteElem { text, properties })
    }
}

#[cfg(test)]
mod quote_elem_tests {
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_quote_elem(rule: Rule, source: &str) -> QuoteElem {
        let mut result = GrammarParser::parse(rule, source).expect("Failed to parse string.");
        let meta = result.next().expect("Failed to parse call expression");
        let meta_ast = QuoteElem::try_from(meta);
        assert!(meta_ast.is_ok());
        meta_ast.expect("Failed to turn pair to `CallExpr` struct")
    }

    #[test]
    fn parses_quote_elem() {
        parse_quote_elem(Rule::quote_decl, r#"> "I'm an example quote""#);
        parse_quote_elem(Rule::narration_decl, r#"* "I'm an example narration""#);
        parse_quote_elem(Rule::choice_decl, r#"- "I'm an example choice""#);
    }
}
