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
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_dialogue_elem(source: &str) -> DialogueElem {
        let mut result =
            GrammarParser::parse(Rule::dialogue_decl, source).expect("Failed to parse string.");
        let element = result.next().expect("Failed to parse element statement");
        let element_ast = DialogueElem::try_from(element);
        assert!(element_ast.is_ok());
        element_ast.expect("Failed to turn pair to `ElementStmt` struct")
    }

    #[test]
    fn parses_dialogue_elem() {
        parse_dialogue_elem(r#"[char] > "I'm a dialogue" > "I'm another dialogue""#);
    }
}
