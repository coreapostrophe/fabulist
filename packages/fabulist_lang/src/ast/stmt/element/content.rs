use pest::iterators::Pair;

use crate::{ast::dfn::object::Object, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct ContentElem {
    pub text: String,
    pub object: Option<Object>,
}

impl TryFrom<Pair<'_, Rule>> for ContentElem {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let mut inner = value.into_inner();

        let text = match inner.find_first_tagged("text") {
            Some(string) => match string.into_inner().next() {
                Some(string_content) => Ok(string_content.as_str().to_string()),
                None => Err(Error::InvalidRule(value_rule)),
            },
            None => Err(Error::InvalidRule(value_rule)),
        }?;
        let object = match inner.find(|pair| pair.as_rule() == Rule::object) {
            Some(object) => Some(Object::try_from(object)?),
            None => None,
        };

        Ok(ContentElem { text, object })
    }
}

#[cfg(test)]
mod content_elem_tests {
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parses_content_elem(rule: Rule, source: &str) -> ContentElem {
        let mut result = GrammarParser::parse(rule, source).expect("Failed to parse string.");
        let content = result.next().expect("Failed to parse content element");
        let content_ast = ContentElem::try_from(content);
        assert!(content_ast.is_ok());
        content_ast.expect("Failed to turn pair to `ContentElem` struct")
    }

    #[test]
    fn parses_dialogue_body() {
        parses_content_elem(Rule::dialogue_body, r#"> "I'm a dialogue""#);
    }

    #[test]
    fn parses_narration() {
        parses_content_elem(Rule::narration, r#"* "I'm a narration""#);
    }

    #[test]
    fn parses_choice() {
        parses_content_elem(Rule::choice, r#"- "I'm a choice""#);
    }
}
