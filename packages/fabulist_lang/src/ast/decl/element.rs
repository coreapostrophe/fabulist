use pest::iterators::Pair;

use crate::parser::Rule;

use self::{dialogue::DialogueDecl, quote::QuoteDecl};

use super::Error;

pub mod dialogue;
pub mod quote;

#[derive(Debug)]
pub enum ElementDecl {
    Dialogue(DialogueDecl),
    Choice(QuoteDecl),
    Narration(QuoteDecl),
}

impl From<DialogueDecl> for ElementDecl {
    fn from(value: DialogueDecl) -> Self {
        Self::Dialogue(value)
    }
}

impl TryFrom<Pair<'_, Rule>> for ElementDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let element_decl_span = value.as_span();

        match value.as_rule() {
            Rule::element_decl => match value.into_inner().next() {
                Some(inner) => Ok(ElementDecl::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::dialogue_decl => Ok(DialogueDecl::try_from(value)?.into()),
            Rule::choice_decl => {
                let content = QuoteDecl::try_from(value)?;
                Ok(ElementDecl::Choice(content))
            }
            Rule::narration_decl => {
                let content = QuoteDecl::try_from(value)?;
                Ok(ElementDecl::Narration(content))
            }
            _ => Err(Error::map_span(element_decl_span, "Invalid declaration")),
        }
    }
}

#[cfg(test)]
mod element_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_element_stmt() {
        let test_helper = ParserTestHelper::<ElementDecl>::new(Rule::element_decl, "ElementDecl");
        test_helper.assert_parse(r#"[char]> "I'm a dialogue""#);
        test_helper.assert_parse(r#"* "I'm a narration""#);
        test_helper.assert_parse(r#"- "I'm a choice""#);
    }
}
