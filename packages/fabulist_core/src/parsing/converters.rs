use std::collections::HashMap;

use fabulist_lang::{
    error::OwnedSpan,
    interpreter::{environment::RuntimeEnvironment, runtime_value::RuntimeValue, Evaluable},
    parser::ast::{
        decl::models::{
            ChoiceElement, DialogueElement, Element, ElementDecl, NarrationElement, PartDecl,
            QuoteDecl,
        },
        story::StoryAst,
    },
};

use crate::{
    error::{EngineError, EngineResult, ParsingError},
    story::{
        context::{ContextValue, Mappable},
        part::{
            dialogue::Dialogue,
            narration::{Narration, NarrationBuilder},
            selection::Selection,
            Part, PartBuilder, PartElement,
        },
        reference::ListKey,
        Story, StoryBuilder,
    },
};

impl From<RuntimeValue> for ContextValue {
    fn from(value: RuntimeValue) -> Self {
        match value {
            RuntimeValue::String { value, .. } => ContextValue::String(value),
            RuntimeValue::Number { value, .. } => ContextValue::Integer(value),
            RuntimeValue::Boolean { value, .. } => ContextValue::Bool(value),
            _ => ContextValue::None,
        }
    }
}

impl From<ContextValue> for RuntimeValue {
    fn from(value: ContextValue) -> Self {
        match value {
            ContextValue::String(string_value) => RuntimeValue::String {
                value: string_value,
                span: OwnedSpan::default(),
            },
            ContextValue::Integer(int_value) => RuntimeValue::Number {
                value: int_value,
                span: OwnedSpan::default(),
            },
            ContextValue::Bool(bool_value) => RuntimeValue::Boolean {
                value: bool_value,
                span: OwnedSpan::default(),
            },
            ContextValue::None => RuntimeValue::None {
                span: OwnedSpan::default(),
            },
        }
    }
}

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
                    obj_dfn.evaluate(&RuntimeEnvironment::new(), &RuntimeEnvironment::new())?
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
    fn try_from(value: NarrationElement) -> Result<Self, Self::Error> {
        let quote = Quote::try_from(value.quote)?;

        let mut narration_builder = NarrationBuilder::new(quote.text);

        if let Some(properties) = quote.properties {
            if let Some(RuntimeValue::Lambda {
                parameters,
                body,
                closure,
                ..
            }) = properties.get("next")
            {
                let parameters = parameters.evaluate(&RuntimeEnvironment::new(), &RuntimeEnvironment::new())?;
                if parameters.is_some_and(|p| !p.is_empty()) {
                    return Err(ParsingError::QueryNextHasParameters);
                }

                let body = body.clone();
                let closure = closure.clone();

                let query_next_closure = Box::new(
                    move |_context: &dyn Mappable| -> EngineResult<ListKey<String>> {
                        let result = body
                            .evaluate(&closure, &RuntimeEnvironment::new())
                            .expect("Failed to evaluate `next` closure in narration.");

                        match result {
                            RuntimeValue::Path { segments, .. } => Ok(ListKey::from(segments)),
                            _ => Err(EngineError::EndOfStory),
                        }
                    },
                );

                narration_builder = narration_builder.set_query_next(query_next_closure);
            }

            if let Some(RuntimeValue::Lambda {
                parameters,
                body,
                closure,
                ..
            }) = properties.get("change_context")
            {
                let parameters = parameters.evaluate(&RuntimeEnvironment::new(), &RuntimeEnvironment::new())?;
                if parameters.is_some_and(|p| !p.is_empty()) {
                    return Err(ParsingError::QueryChangeContextHasParameters);
                }

                let _body = body.clone();
                let _closure = closure.clone();

                let change_context_closure =
                    Box::new(move |_context: &mut dyn Mappable| -> EngineResult<()> { todo!() });

                narration_builder = narration_builder.set_change_context(change_context_closure);
            }
        }

        Ok(narration_builder.build())
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
            .evaluate(&RuntimeEnvironment::new(), &RuntimeEnvironment::new())?
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
