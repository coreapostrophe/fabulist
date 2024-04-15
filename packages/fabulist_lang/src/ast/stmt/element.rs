use pest::iterators::Pair;

use crate::parser::Rule;

use self::{content::ContentElem, dialogue::DialogueElem};

use super::Error;

pub mod content;
pub mod dialogue;

pub enum ElementStmt {
    Dialogue(DialogueElem),
    Choice(ContentElem),
    Narration(ContentElem),
}

impl From<DialogueElem> for ElementStmt {
    fn from(value: DialogueElem) -> Self {
        Self::Dialogue(value)
    }
}

impl TryFrom<Pair<'_, Rule>> for ElementStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();

        match value_rule {
            Rule::element => match value.into_inner().next() {
                Some(inner) => Ok(ElementStmt::try_from(inner)?),
                None => Err(Error::InvalidRule(value_rule)),
            },
            Rule::dialogue => Ok(DialogueElem::try_from(value)?.into()),
            Rule::choice => {
                let content = ContentElem::try_from(value)?;
                Ok(ElementStmt::Choice(content))
            }
            Rule::narration => {
                let content = ContentElem::try_from(value)?;
                Ok(ElementStmt::Narration(content))
            }
            _ => Err(Error::InvalidRule(value_rule)),
        }
    }
}

#[cfg(test)]
mod element_stmt_tests {
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_element_stmt(source: &str) -> ElementStmt {
        let mut result =
            GrammarParser::parse(Rule::element, source).expect("Failed to parse string.");
        let element = result.next().expect("Failed to parse element statement");
        let element_ast = ElementStmt::try_from(element);
        assert!(element_ast.is_ok());
        element_ast.expect("Failed to turn pair to `ElementStmt` struct")
    }

    #[test]
    fn parses_element_stmt() {
        parse_element_stmt(r#"[char]> "I'm a dialogue""#);
        parse_element_stmt(r#"* "I'm a narration""#);
        parse_element_stmt(r#"- "I'm a choice""#);
    }
}
