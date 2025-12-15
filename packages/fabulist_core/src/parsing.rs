#![cfg(feature = "parsing")]
use fabulist_lang::interpreter::{environment::RuntimeEnvironment, runtime_value::RuntimeValue};

use crate::{
    error::ParsingResult,
    story::{
        context::{ContextValue, Mappable},
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

impl Mappable for RuntimeEnvironment {
    fn insert(&mut self, key: String, value: crate::story::context::ContextValue) {
        let runtime_value: RuntimeValue = value.into();
        self.insert_env_value(key, runtime_value)
            .expect("Failed to insert env value");
    }
    fn get(&self, key: &str) -> Option<&crate::story::context::ContextValue> {
        if let Some(runtime_value) = self.get_env_value(key) {
            let context_value: ContextValue = runtime_value.clone().into();
            Some(Box::leak(Box::new(context_value)))
        } else {
            None
        }
    }
    fn assign(&mut self, key: String, new_value: crate::story::context::ContextValue) -> bool {
        let runtime_value: RuntimeValue = new_value.into();
        let result = self.assign_env_value(key, runtime_value);
        result.is_ok()
    }
}
