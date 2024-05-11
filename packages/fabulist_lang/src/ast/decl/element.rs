use fabulist_derive::SyntaxTree;
use pest::{error::LineColLocation, iterators::Pair};

use crate::{
    ast::decl::{DialogueDecl, QuoteDecl},
    error::Error,
    parser::Rule,
};

#[derive(SyntaxTree, Debug, Clone)]
pub enum Element {
    #[production(value: DialogueDecl)]
    Dialogue(DialogueElement),

    #[production(value: QuoteDecl)]
    Choice(ChoiceElement),

    #[production(value: QuoteDecl)]
    Narration(NarrationElement),
}

impl TryFrom<Pair<'_, Rule>> for Element {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);

        match value.as_rule() {
            Rule::element_decl => match value.into_inner().next() {
                Some(inner) => Ok(Element::try_from(inner)?),
                None => Err(Error::map_span(
                    value_span,
                    "Unable to parse token tree interior",
                )),
            },
            Rule::dialogue_decl => Ok(Element::Dialogue(DialogueElement {
                lcol: value_lcol,
                value: DialogueDecl::try_from(value)?,
            })),
            Rule::choice_decl => Ok(Element::Choice(ChoiceElement {
                lcol: value_lcol,
                value: QuoteDecl::try_from(value)?,
            })),
            Rule::narration_decl => Ok(Element::Narration(NarrationElement {
                lcol: value_lcol,
                value: QuoteDecl::try_from(value)?,
            })),
            _ => Err(Error::map_span(value_span, "Invalid declaration")),
        }
    }
}

#[cfg(test)]
mod element_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_element_stmt() {
        let test_helper = ParserTestHelper::<Element>::new(Rule::element_decl, "ElementDecl");
        test_helper.assert_parse(r#"[char]> "I'm a dialogue""#);
        test_helper.assert_parse(r#"* "I'm a narration""#);
        test_helper.assert_parse(r#"- "I'm a choice""#);
    }
}
