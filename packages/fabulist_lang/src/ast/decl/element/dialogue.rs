use pest::{error::LineColLocation, iterators::Pair};

use crate::parser::Rule;

use super::{quote::QuoteDecl, Error};

#[derive(Debug, Clone)]
pub struct DialogueDecl {
    pub lcol: LineColLocation,
    pub character: String,
    pub quotes: Vec<QuoteDecl>,
}

impl TryFrom<Pair<'_, Rule>> for DialogueDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let dialogue_decl_span = value.as_span();
        let dialogue_decl_lcol = LineColLocation::from(dialogue_decl_span);
        let inner = value.into_inner();

        let character = match inner.find_first_tagged("character") {
            Some(char) => Ok(match char.into_inner().next() {
                Some(char) => Ok(char.as_str().to_string()),
                None => Err(Error::map_span(dialogue_decl_span, "Expected string value")),
            }?),
            None => Err(Error::map_span(
                dialogue_decl_span,
                "Expected character declaration",
            )),
        }?;

        let quotes = inner
            .filter(|pair| pair.as_rule() == Rule::quote_decl)
            .map(QuoteDecl::try_from)
            .collect::<Result<Vec<QuoteDecl>, Error>>()?;

        Ok(DialogueDecl {
            character,
            quotes,
            lcol: dialogue_decl_lcol,
        })
    }
}

#[cfg(test)]
mod dialogue_elem_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_dialogue_elem() {
        let test_helper =
            ParserTestHelper::<DialogueDecl>::new(Rule::dialogue_decl, "DialogueDecl");
        test_helper.assert_parse(r#"[char] > "I'm a dialogue" > "I'm another dialogue""#);
    }
}
