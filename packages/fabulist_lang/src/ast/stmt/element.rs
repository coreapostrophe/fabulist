use pest::iterators::Pair;

use crate::parser::Rule;

use self::{dialogue::DialogueElem, quote::QuoteElem};

use super::Error;

pub mod dialogue;
pub mod quote;

#[derive(Debug)]
pub enum ElementStmt {
    Dialogue(DialogueElem),
    Choice(QuoteElem),
    Narration(QuoteElem),
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
            Rule::element_decl => match value.into_inner().next() {
                Some(inner) => Ok(ElementStmt::try_from(inner)?),
                None => Err(Error::InvalidRule(value_rule)),
            },
            Rule::dialogue_decl => Ok(DialogueElem::try_from(value)?.into()),
            Rule::choice_decl => {
                let content = QuoteElem::try_from(value)?;
                Ok(ElementStmt::Choice(content))
            }
            Rule::narration_decl => {
                let content = QuoteElem::try_from(value)?;
                Ok(ElementStmt::Narration(content))
            }
            _ => Err(Error::InvalidRule(value_rule)),
        }
    }
}

#[cfg(test)]
mod element_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_element_stmt() {
        let test_helper = ParserTestHelper::<ElementStmt>::new(Rule::element_decl, "ElementStmt");
        test_helper.assert_parse(r#"[char]> "I'm a dialogue""#);
        test_helper.assert_parse(r#"* "I'm a narration""#);
        test_helper.assert_parse(r#"- "I'm a choice""#);
    }
}
