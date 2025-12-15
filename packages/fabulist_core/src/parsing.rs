#![cfg(feature = "parsing")]
use fabulist_lang::interpreter::{
    environment::{Environment, RuntimeEnvironment},
    runtime_value::RuntimeValue,
};

use crate::{
    error::ParsingResult,
    story::{
        context::{ContextValue, Contextual},
        StoryBuilder,
    },
};

pub mod converters;

impl StoryBuilder {
    pub fn parse(source: impl Into<String>) -> ParsingResult<Self> {
        use crate::error::ParsingError;

        let _story_ast = fabulist_lang::parser::FabulistParser::parse(source.into())
            .map_err(|err| ParsingError::from(Box::new(*err)))?;

        todo!()
    }
}

impl Contextual for RuntimeEnvironment {
    fn insert(&mut self, key: String, value: crate::story::context::ContextValue) {
        let runtime_value: RuntimeValue = value.into();
        Environment::insert(self, key, runtime_value);
    }
    fn get(&self, key: &str) -> Option<&crate::story::context::ContextValue> {
        if let Some(runtime_value) = Environment::get_value(self, key) {
            let context_value: ContextValue = runtime_value.clone().into();
            Some(Box::leak(Box::new(context_value)))
        } else {
            None
        }
    }
    fn assign(&mut self, key: String, new_value: crate::story::context::ContextValue) -> bool {
        let runtime_value: RuntimeValue = new_value.into();
        let result = Environment::assign(self, key, runtime_value);
        result.is_ok()
    }
}
