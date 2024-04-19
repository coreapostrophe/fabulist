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
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_quote_elem() {
        let test_helper = ParserTestHelper::<QuoteElem>::new(Rule::quote_decl, "QuoteDecl");
        test_helper.assert_parse(r#"> "I'm an example quote""#);

        let test_helper = ParserTestHelper::<QuoteElem>::new(Rule::narration_decl, "QuoteDecl");
        test_helper.assert_parse(r#"* "I'm an example narration""#);

        let test_helper = ParserTestHelper::<QuoteElem>::new(Rule::choice_decl, "QuoteDecl");
        test_helper.assert_parse(r#"- "I'm an example choice""#);
    }
}
