use pest::iterators::Pair;

use crate::{
    ast::{
        dfn::models::ObjectDfn,
        expr::models::{IdentifierPrimitive, Literal, StringLiteral},
    },
    error::ParsingError,
    parser::Rule,
};

use super::models::{
    ChoiceElement, Decl, DialogueDecl, DialogueElement, Element, ElementDecl, MetaDecl, ModuleDecl,
    NarrationElement, PartDecl, QuoteDecl,
};

impl TryFrom<Pair<'_, Rule>> for Decl {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        match value.as_rule() {
            Rule::quote_decl => Ok(Decl::Quote(QuoteDecl::try_from(value)?)),
            Rule::dialogue_decl => Ok(Decl::Dialogue(DialogueDecl::try_from(value)?)),
            Rule::element_decl => Ok(Decl::Element(ElementDecl::try_from(value)?)),
            Rule::meta_decl => Ok(Decl::Meta(MetaDecl::try_from(value)?)),
            Rule::mod_decl => Ok(Decl::Module(ModuleDecl::try_from(value)?)),
            Rule::part_decl => Ok(Decl::Part(PartDecl::try_from(value)?)),
            _ => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Invalid declaration",
            )),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for QuoteDecl {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let mut inner = value.into_inner();

        let text = match inner.find(|pair| pair.as_node_tag() == Some("text")) {
            Some(text) => Ok(match text.into_inner().next() {
                Some(text) => Ok(text.as_str().to_string()),
                None => Err(ParsingError::map_custom_error(
                    value_span.into(),
                    "Expected string value",
                )),
            }?),
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected text expression",
            )),
        }?;

        let properties = match inner.find(|pair| pair.as_rule() == Rule::object) {
            Some(object) => Some(ObjectDfn::try_from(object)?),
            None => None,
        };

        Ok(QuoteDecl {
            span: value_span.into(),
            text,
            properties,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for DialogueDecl {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let inner = value.into_inner();

        let character = match inner.find_first_tagged("character") {
            Some(char) => Ok(match char.into_inner().next() {
                Some(char) => Ok(char.as_str().to_string()),
                None => Err(ParsingError::map_custom_error(
                    value_span.into(),
                    "Expected string value",
                )),
            }?),
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected character declaration",
            )),
        }?;

        let quotes = inner
            .filter(|pair| pair.as_rule() == Rule::quote_decl)
            .map(QuoteDecl::try_from)
            .collect::<Result<Vec<QuoteDecl>, pest::error::Error<Rule>>>()?;

        Ok(DialogueDecl {
            span: value_span.into(),
            character,
            quotes,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for ElementDecl {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        Ok(ElementDecl {
            span: value_span.into(),
            value: Element::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for MetaDecl {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        match value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::object)
        {
            Some(object) => Ok(MetaDecl {
                span: value_span.into(),
                properties: ObjectDfn::try_from(object)?,
            }),
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected object definition",
            )),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ModuleDecl {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let mut inner = value.into_inner();

        let path = match inner
            .clone()
            .find(|pair| pair.as_node_tag() == Some("path"))
        {
            Some(path) => match Literal::try_from(path)? {
                Literal::String(StringLiteral { value, .. }) => Ok(value),
                _ => Err(ParsingError::map_custom_error(
                    value_span.into(),
                    "Expected string",
                )),
            },
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected string file path",
            )),
        }?;

        let identifier = match inner.find(|pair| pair.as_rule() == Rule::identifier) {
            Some(identifier) => IdentifierPrimitive::try_from(identifier),
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected identifier",
            )),
        }?;

        Ok(ModuleDecl {
            span: value_span.into(),
            path,
            identifier,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for PartDecl {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let mut inner = value.into_inner();

        let id = match inner.find(|pair| pair.as_node_tag() == Some("id")) {
            Some(id) => match id
                .into_inner()
                .find(|pair| pair.as_node_tag() == Some("name"))
            {
                Some(identifier) => Ok(identifier.as_str().to_string()),
                None => Err(ParsingError::map_custom_error(
                    value_span.into(),
                    "Expected identifier",
                )),
            },
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected id declaration",
            )),
        }?;
        let elements = inner
            .filter(|pair| pair.as_rule() == Rule::element_decl)
            .map(ElementDecl::try_from)
            .collect::<Result<Vec<ElementDecl>, pest::error::Error<Rule>>>()?;

        Ok(PartDecl {
            span: value_span.into(),
            id,
            elements,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for Element {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        match value.as_rule() {
            Rule::element_decl => match value.into_inner().next() {
                Some(inner) => Ok(Element::try_from(inner)?),
                None => Err(ParsingError::map_custom_error(
                    value_span.into(),
                    "Unable to parse token tree interior",
                )),
            },
            Rule::dialogue_decl => Ok(Element::Dialogue(DialogueElement {
                span: value_span.into(),
                value: DialogueDecl::try_from(value)?,
            })),
            Rule::choice_decl => Ok(Element::Choice(ChoiceElement {
                span: value_span.into(),
                value: QuoteDecl::try_from(value)?,
            })),
            Rule::narration_decl => Ok(Element::Narration(NarrationElement {
                span: value_span.into(),
                value: QuoteDecl::try_from(value)?,
            })),
            _ => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Invalid declaration",
            )),
        }
    }
}
