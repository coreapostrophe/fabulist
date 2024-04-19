use pest::iterators::Pair;

use crate::parser::Rule;

use super::{quote::QuoteElem, Error};

#[derive(Debug)]
pub struct DialogueElem {
    pub character: String,
    pub quotes: Vec<QuoteElem>,
}

impl TryFrom<Pair<'_, Rule>> for DialogueElem {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let inner = value.into_inner();

        let character = match inner.find_first_tagged("character") {
            Some(char) => Ok(match char.into_inner().next() {
                Some(char) => Ok(char.as_str().to_string()),
                None => Err(Error::InvalidRule(value_rule)),
            }?),
            None => Err(Error::InvalidRule(value_rule)),
        }?;

        let quotes = inner
            .filter(|pair| pair.as_rule() == Rule::quote_decl)
            .map(|pair| QuoteElem::try_from(pair))
            .collect::<Result<Vec<QuoteElem>, Error>>()?;

        Ok(DialogueElem { character, quotes })
    }
}

#[cfg(test)]
mod dialogue_elem_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_dialogue_elem() {
        let test_helper =
            ParserTestHelper::<DialogueElem>::new(Rule::dialogue_decl, "DialogueElem");
        test_helper.assert_parse(r#"[char] > "I'm a dialogue" > "I'm another dialogue""#);
    }
}
