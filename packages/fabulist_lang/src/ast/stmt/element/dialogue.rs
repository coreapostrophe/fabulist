use pest::iterators::Pair;

use crate::parser::Rule;

use super::{content::ContentElem, Error};

pub struct DialogueElem {
    pub character: String,
    pub dialogues: Vec<ContentElem>,
}

impl TryFrom<Pair<'_, Rule>> for DialogueElem {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let inner = value.into_inner();

        let character = match inner.find_first_tagged("char") {
            Some(character) => match character.into_inner().find_first_tagged("name") {
                Some(character_name) => Ok(character_name.as_str().to_string()),
                None => Err(Error::InvalidRule(value_rule)),
            },
            None => Err(Error::InvalidRule(value_rule)),
        }?;
        let dialogues = inner
            .filter(|pair| pair.as_rule() == Rule::dialogue_body)
            .map(|pair| ContentElem::try_from(pair))
            .collect::<Result<Vec<ContentElem>, Error>>()?;

        Ok(DialogueElem {
            character,
            dialogues,
        })
    }
}

#[cfg(test)]
mod dialogue_elem_tests {
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_dialogue_elem(source: &str) -> DialogueElem {
        let mut result =
            GrammarParser::parse(Rule::dialogue, source).expect("Failed to parse string.");
        let dialogue = result.next().expect("Failed to parse dialogue element");
        let dialogue_ast = DialogueElem::try_from(dialogue);
        assert!(dialogue_ast.is_ok());
        dialogue_ast.expect("Failed to turn pair to `DialogueElem` struct")
    }

    #[test]
    fn parses_dialogue_elem() {
        parse_dialogue_elem(
            r#"[char]
            > "I'm a dialogue"
            > "I'm a dialogue also"
            > "I'm a dialogue as well"
            "#,
        );
    }
}
