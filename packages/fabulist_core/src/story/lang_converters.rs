use std::collections::HashMap;

use fabulist_lang::{
    interpreter::environment::Environment,
    interpreter::{runtime_value::RuntimeValue, Evaluable},
    parser::ast::{
        decl::models::{
            ChoiceElement, DialogueElement, Element, ElementDecl, NarrationElement, PartDecl,
            QuoteDecl,
        },
        story::StoryAst,
    },
};

use crate::{
    error::ParsingError,
    story::{
        part::{
            dialogue::Dialogue, narration::Narration, selection::Selection, Part, PartBuilder,
            PartElement,
        },
        Story, StoryBuilder,
    },
};

pub struct Quote {
    pub text: String,
    pub properties: Option<HashMap<String, RuntimeValue>>,
}

impl TryFrom<QuoteDecl> for Quote {
    type Error = ParsingError;
    fn try_from(value: QuoteDecl) -> Result<Self, Self::Error> {
        let properties = match value.properties {
            Some(obj_dfn) => {
                let RuntimeValue::Object { properties, .. } =
                    obj_dfn.evaluate(&Environment::new(), &Environment::new())?
                else {
                    return Err(ParsingError::InvalidQuoteProperties);
                };
                Some(properties)
            }
            None => None,
        };

        Ok(Self {
            text: value.text,
            properties,
        })
    }
}

impl TryFrom<DialogueElement> for Dialogue {
    type Error = ParsingError;
    fn try_from(_value: DialogueElement) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<ChoiceElement> for Selection {
    type Error = ParsingError;
    fn try_from(_value: ChoiceElement) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<NarrationElement> for Narration {
    type Error = ParsingError;
    fn try_from(_value: NarrationElement) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<ElementDecl> for Box<PartElement> {
    type Error = ParsingError;
    fn try_from(element_decl: ElementDecl) -> Result<Self, Self::Error> {
        match element_decl.value {
            Element::Dialogue(dialogue_element) => {
                let dialogue = Dialogue::try_from(dialogue_element)?;
                Ok(Box::new(dialogue))
            }
            Element::Choice(choice_element) => {
                let selection = Selection::try_from(choice_element)?;
                Ok(Box::new(selection))
            }
            Element::Narration(narration_element) => {
                let narration = Narration::try_from(narration_element)?;
                Ok(Box::new(narration))
            }
        }
    }
}

impl TryFrom<PartDecl> for Part {
    type Error = ParsingError;
    fn try_from(value: PartDecl) -> Result<Self, Self::Error> {
        let mut part_builder = PartBuilder::new(value.id);

        for element_decl in value.elements {
            let element: Box<PartElement> = element_decl.try_into()?;
            part_builder = part_builder.add_element(element);
        }

        Ok(part_builder.build())
    }
}

impl TryFrom<StoryAst> for Story {
    type Error = ParsingError;
    fn try_from(value: StoryAst) -> Result<Self, Self::Error> {
        let mut builder = StoryBuilder::new();

        for part_decl in value.parts {
            let part: Part = part_decl.try_into()?;
            builder = builder.add_part(part);
        }

        let Some(meta) = value.meta else {
            return Err(ParsingError::StartMetadataRequired);
        };

        let RuntimeValue::Object { properties, .. } = meta
            .properties
            .evaluate(&Environment::new(), &Environment::new())?
        else {
            return Err(ParsingError::StartMetadataRequired);
        };

        let Some(RuntimeValue::String {
            value: start_value, ..
        }) = properties.get("start")
        else {
            return Err(ParsingError::StartMetadataRequired);
        };

        builder = builder.set_start(start_value);

        Ok(builder.build())
    }
}
