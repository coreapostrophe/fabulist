use crate::{
    ast::{
        dfn::ObjectDfn,
        expr::{
            literal::{Literal, StringLiteral},
            primitive::IdentifierPrimitive,
        },
    },
    error::Error,
    parser::Rule,
};
use fabulist_derive::SyntaxTree;
use pest::{error::LineColLocation, iterators::Pair};

pub mod element;

use element::Element;

#[derive(SyntaxTree, Debug, Clone)]
pub enum Decl {
    #[production(text: String, properties: Option<ObjectDfn>)]
    Quote(QuoteDecl),

    #[production(character: String, quotes: Vec<QuoteDecl>)]
    Dialogue(DialogueDecl),

    #[production(value: Element)]
    Element(ElementDecl),

    #[production(properties: ObjectDfn)]
    Meta(MetaDecl),

    #[production(path: String, identifier: IdentifierPrimitive)]
    Module(ModuleDecl),

    #[production(id: String, elements: Vec<ElementDecl>)]
    Part(PartDecl),
}

impl TryFrom<Pair<'_, Rule>> for Decl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        match value.as_rule() {
            Rule::quote_decl => Ok(Decl::Quote(QuoteDecl::try_from(value)?)),
            Rule::dialogue_decl => Ok(Decl::Dialogue(DialogueDecl::try_from(value)?)),
            Rule::element_decl => Ok(Decl::Element(ElementDecl::try_from(value)?)),
            Rule::meta_decl => Ok(Decl::Meta(MetaDecl::try_from(value)?)),
            Rule::mod_decl => Ok(Decl::Module(ModuleDecl::try_from(value)?)),
            Rule::part_decl => Ok(Decl::Part(PartDecl::try_from(value)?)),
            _ => Err(Error::map_span(value_span, "Invalid declaration")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for QuoteDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let mut inner = value.into_inner();

        let text = match inner.find(|pair| pair.as_node_tag() == Some("text")) {
            Some(text) => Ok(match text.into_inner().next() {
                Some(text) => Ok(text.as_str().to_string()),
                None => Err(Error::map_span(value_span, "Expected string value")),
            }?),
            None => Err(Error::map_span(value_span, "Expected text expression")),
        }?;

        let properties = match inner.find(|pair| pair.as_rule() == Rule::object) {
            Some(object) => Some(ObjectDfn::try_from(object)?),
            None => None,
        };

        Ok(QuoteDecl {
            lcol: value_lcol,
            text,
            properties,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for DialogueDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let inner = value.into_inner();

        let character = match inner.find_first_tagged("character") {
            Some(char) => Ok(match char.into_inner().next() {
                Some(char) => Ok(char.as_str().to_string()),
                None => Err(Error::map_span(value_span, "Expected string value")),
            }?),
            None => Err(Error::map_span(
                value_span,
                "Expected character declaration",
            )),
        }?;

        let quotes = inner
            .filter(|pair| pair.as_rule() == Rule::quote_decl)
            .map(QuoteDecl::try_from)
            .collect::<Result<Vec<QuoteDecl>, Error>>()?;

        Ok(DialogueDecl {
            lcol: value_lcol,
            character,
            quotes,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for ElementDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());
        Ok(ElementDecl {
            lcol: value_lcol,
            value: Element::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for MetaDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        match value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::object)
        {
            Some(object) => Ok(MetaDecl {
                lcol: value_lcol,
                properties: ObjectDfn::try_from(object)?,
            }),
            None => Err(Error::map_span(value_span, "Expected object definition")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ModuleDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let mut inner = value.into_inner();

        let path = match inner
            .clone()
            .find(|pair| pair.as_node_tag() == Some("path"))
        {
            Some(path) => match Literal::try_from(path)? {
                Literal::String(StringLiteral { value, .. }) => Ok(value),
                _ => Err(Error::map_span(value_span, "Expected string")),
            },
            None => Err(Error::map_span(value_span, "Expected string file path")),
        }?;

        let identifier = match inner.find(|pair| pair.as_rule() == Rule::identifier) {
            Some(identifier) => IdentifierPrimitive::try_from(identifier),
            None => Err(Error::map_span(value_span, "Expected identifier")),
        }?;

        Ok(ModuleDecl {
            path,
            identifier,
            lcol: value_lcol,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for PartDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let mut inner = value.into_inner();

        let id = match inner.find(|pair| pair.as_node_tag() == Some("id")) {
            Some(id) => match id
                .into_inner()
                .find(|pair| pair.as_node_tag() == Some("name"))
            {
                Some(identifier) => Ok(identifier.as_str().to_string()),
                None => Err(Error::map_span(value_span, "Expected identifier")),
            },
            None => Err(Error::map_span(value_span, "Expected id declaration")),
        }?;
        let elements = inner
            .filter(|pair| pair.as_rule() == Rule::element_decl)
            .map(ElementDecl::try_from)
            .collect::<Result<Vec<ElementDecl>, Error>>()?;

        Ok(PartDecl {
            lcol: value_lcol,
            id,
            elements,
        })
    }
}

#[cfg(test)]
mod decl_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_quote_elem() {
        let test_helper = ParserTestHelper::<QuoteDecl>::new(Rule::quote_decl, "QuoteDecl");
        test_helper.assert_parse(r#"> "I'm an example quote""#);

        let test_helper = ParserTestHelper::<QuoteDecl>::new(Rule::narration_decl, "QuoteDecl");
        test_helper.assert_parse(r#"* "I'm an example narration""#);

        let test_helper = ParserTestHelper::<QuoteDecl>::new(Rule::choice_decl, "QuoteDecl");
        test_helper.assert_parse(r#"- "I'm an example choice""#);
    }

    #[test]
    fn parses_dialogue_elem() {
        let test_helper =
            ParserTestHelper::<DialogueDecl>::new(Rule::dialogue_decl, "DialogueDecl");
        test_helper.assert_parse(r#"[char] > "I'm a dialogue" > "I'm another dialogue""#);
    }

    #[test]
    fn parses_element_stmt() {
        let test_helper = ParserTestHelper::<Element>::new(Rule::element_decl, "ElementDecl");
        test_helper.assert_parse(r#"[char]> "I'm a dialogue""#);
        test_helper.assert_parse(r#"* "I'm a narration""#);
        test_helper.assert_parse(r#"- "I'm a choice""#);
    }

    #[test]
    fn parses_meta_stmt() {
        let test_helper = ParserTestHelper::<MetaDecl>::new(Rule::meta_decl, "MetaDecl");
        test_helper.assert_parse(r#"story { "start": "part-1" }"#);
    }

    #[test]
    fn parses_module_tests() {
        let test_helper = ParserTestHelper::<ModuleDecl>::new(Rule::mod_decl, "ModDecl");
        test_helper.assert_parse("module \"./module.fab\" as module_1;");
    }

    #[test]
    fn parses_part_stmt() {
        let test_helper = ParserTestHelper::<PartDecl>::new(Rule::part_decl, "PartDecl");
        test_helper.assert_parse(r#"#ident-1 [char]>"I'm a dialogue""#);
    }
}
