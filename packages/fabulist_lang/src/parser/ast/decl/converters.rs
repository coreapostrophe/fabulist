//! Converters from pest parse pairs into declaration AST nodes.
use pest::iterators::Pair;

use crate::parser::{
    ast::{
        dfn::models::ObjectDfn,
        expr::models::{IdentifierPrimitive, Literal, StringLiteral},
    },
    error::{ExtractSpanSlice, ParserError},
    Rule,
};

use super::models::{
    ChoiceElement, Decl, DialogueDecl, DialogueElement, Element, ElementDecl, MetaDecl, ModuleDecl,
    NarrationElement, PartDecl, QuoteDecl,
};

impl TryFrom<Pair<'_, Rule>> for QuoteDecl {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let text = match inner.find(|pair| pair.as_node_tag() == Some("text")) {
            Some(text) => {
                let text_value_span = text.extract_span_slice();

                Ok(match text.into_inner().next() {
                    Some(text_string) => Ok(text_string.as_str().to_string()),
                    None => Err(ParserError::ExpectedSymbol {
                        expected: "text string".to_string(),
                        span_slice: text_value_span,
                    }),
                }?)
            }
            None => Err(ParserError::ExpectedSymbol {
                expected: "quote text".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        let properties = match inner.find(|pair| pair.as_rule() == Rule::object) {
            Some(object) => Some(ObjectDfn::try_from(object)?),
            None => None,
        };

        Ok(QuoteDecl {
            span_slice: value_span_slice,
            text,
            properties,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for DialogueDecl {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        let inner = value.into_inner();

        let character = match inner.find_first_tagged("character") {
            Some(character) => Ok(match character.into_inner().next() {
                Some(character_identifier) => Ok(character_identifier.as_str().to_string()),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "character identifier".to_string(),
                    span_slice: value_span_slice.clone(),
                }),
            }?),
            None => Err(ParserError::ExpectedSymbol {
                expected: "dialogue character".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        let quotes = inner
            .filter(|pair| pair.as_rule() == Rule::quote_decl)
            .map(QuoteDecl::try_from)
            .collect::<Result<Vec<QuoteDecl>, ParserError>>()?;

        Ok(DialogueDecl {
            span_slice: value_span_slice,
            character,
            quotes,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for ElementDecl {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        Ok(ElementDecl {
            span_slice: value_span_slice,
            value: Element::try_from(value)?,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for MetaDecl {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let span_slice = value.extract_span_slice();

        match value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::object)
        {
            Some(object) => Ok(MetaDecl {
                span_slice,
                properties: ObjectDfn::try_from(object)?,
            }),
            None => Err(ParserError::ExpectedSymbol {
                expected: "object".to_string(),
                span_slice,
            }),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ModuleDecl {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let path = match inner
            .clone()
            .find(|pair| pair.as_node_tag() == Some("path"))
        {
            Some(path) => match Literal::try_from(path)? {
                Literal::String(StringLiteral { value, .. }) => Ok(value),
                _ => Err(ParserError::ExpectedSymbol {
                    expected: "path to be a string".to_string(),
                    span_slice: value_span_slice.clone(),
                }),
            },
            None => Err(ParserError::ExpectedSymbol {
                expected: "string file path".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        let identifier = match inner.find(|pair| pair.as_rule() == Rule::identifier) {
            Some(identifier) => IdentifierPrimitive::try_from(identifier),
            None => Err(ParserError::ExpectedSymbol {
                expected: "module identifier".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        Ok(ModuleDecl {
            span_slice: value_span_slice,
            path,
            identifier,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for PartDecl {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let id = match inner.find(|pair| pair.as_node_tag() == Some("id")) {
            Some(id) => match id
                .into_inner()
                .find(|pair| pair.as_node_tag() == Some("name"))
            {
                Some(identifier) => Ok(identifier.as_str().to_string()),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "identifier".to_string(),
                    span_slice: value_span_slice.clone(),
                }),
            },
            None => Err(ParserError::ExpectedSymbol {
                expected: "id declaration".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;
        let elements = inner
            .filter(|pair| pair.as_rule() == Rule::element_decl)
            .map(ElementDecl::try_from)
            .collect::<Result<Vec<ElementDecl>, ParserError>>()?;

        Ok(PartDecl {
            span_slice: value_span_slice,
            id,
            elements,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for Decl {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::quote_decl => Ok(Decl::Quote(QuoteDecl::try_from(value)?)),
            Rule::dialogue_decl => Ok(Decl::Dialogue(DialogueDecl::try_from(value)?)),
            Rule::element_decl => Ok(Decl::Element(ElementDecl::try_from(value)?)),
            Rule::meta_decl => Ok(Decl::Meta(MetaDecl::try_from(value)?)),
            Rule::mod_decl => Ok(Decl::Module(ModuleDecl::try_from(value)?)),
            Rule::part_decl => Ok(Decl::Part(PartDecl::try_from(value)?)),
            _ => Err(ParserError::ExpectedSymbol {
                expected: "Invalid declaration".to_string(),
                span_slice: value.extract_span_slice(),
            }),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Element {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        match value.as_rule() {
            Rule::element_decl => match value.into_inner().next() {
                Some(inner) => Ok(Element::try_from(inner)?),
                None => Err(ParserError::ExpectedSymbol {
                    expected: "Unable to parse token tree interior".to_string(),
                    span_slice: value_span_slice.clone(),
                }),
            },
            Rule::dialogue_decl => Ok(Element::Dialogue(DialogueElement {
                span_slice: value_span_slice.clone(),
                value: DialogueDecl::try_from(value)?,
            })),
            Rule::choice_decl => Ok(Element::Choice(ChoiceElement {
                span_slice: value_span_slice.clone(),
                quote: QuoteDecl::try_from(value)?,
            })),
            Rule::narration_decl => Ok(Element::Narration(NarrationElement {
                span_slice: value_span_slice.clone(),
                quote: QuoteDecl::try_from(value)?,
            })),
            _ => Err(ParserError::ExpectedSymbol {
                expected: "Invalid declaration".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::ast::AstTestHelper, parser::Rule};

    use super::*;

    #[test]
    fn parses_quote_elem() {
        let test_helper = AstTestHelper::<QuoteDecl>::new(Rule::quote_decl, "QuoteDecl");
        test_helper.assert_parse(r#"> "I'm an example quote""#);

        let test_helper = AstTestHelper::<QuoteDecl>::new(Rule::narration_decl, "QuoteDecl");
        test_helper.assert_parse(r#"* "I'm an example narration""#);

        let test_helper = AstTestHelper::<QuoteDecl>::new(Rule::choice_decl, "QuoteDecl");
        test_helper.assert_parse(r#"- "I'm an example choice""#);
    }

    #[test]
    fn parses_dialogue_elem() {
        let test_helper = AstTestHelper::<DialogueDecl>::new(Rule::dialogue_decl, "DialogueDecl");
        test_helper.assert_parse(r#"[char] > "I'm a dialogue" > "I'm another dialogue""#);
    }

    #[test]
    fn parses_element_stmt() {
        let test_helper = AstTestHelper::<Element>::new(Rule::element_decl, "ElementDecl");
        test_helper.assert_parse(r#"[char]> "I'm a dialogue""#);
        test_helper.assert_parse(r#"* "I'm a narration""#);
        test_helper.assert_parse(r#"- "I'm a choice""#);
    }

    #[test]
    fn parses_meta_stmt() {
        let test_helper = AstTestHelper::<MetaDecl>::new(Rule::meta_decl, "MetaDecl");
        test_helper.assert_parse(r#"story { "start": "part-1" }"#);
    }

    #[test]
    fn parses_module_tests() {
        let test_helper = AstTestHelper::<ModuleDecl>::new(Rule::mod_decl, "ModDecl");
        test_helper.assert_parse("module \"./module.fab\" as module_1;");
    }

    #[test]
    fn parses_part_stmt() {
        let test_helper = AstTestHelper::<PartDecl>::new(Rule::part_decl, "PartDecl");
        test_helper.assert_parse(r#"#ident-1 [char]>"I'm a dialogue""#);
    }
}
