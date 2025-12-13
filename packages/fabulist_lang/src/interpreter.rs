use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{dfn::models::ParameterBodyDfn, stmt::models::BlockStmt},
    context::Context,
    environment::Environment,
    error::{OwnedSpan, RuntimeError},
};

#[derive(Clone, Debug)]
pub enum RuntimeValue {
    Number {
        value: f32,
        span: OwnedSpan,
    },
    Boolean {
        value: bool,
        span: OwnedSpan,
    },
    String {
        value: String,
        span: OwnedSpan,
    },
    Object {
        properties: HashMap<String, RuntimeValue>,
        span: OwnedSpan,
    },
    Lambda {
        parameters: ParameterBodyDfn,
        body: BlockStmt,
        closure: Rc<RefCell<Environment>>,
    },
    NativeFunction(fn(Vec<RuntimeValue>, OwnedSpan) -> Result<RuntimeValue, RuntimeError>),
    Identifier {
        name: String,
        span: OwnedSpan,
    },
    None,
    Context,
}

impl RuntimeValue {
    pub fn type_name(&self) -> String {
        match self {
            RuntimeValue::Number { .. } => "Number".to_string(),
            RuntimeValue::Boolean { .. } => "Boolean".to_string(),
            RuntimeValue::String { .. } => "String".to_string(),
            RuntimeValue::Identifier { .. } => "Identifier".to_string(),
            RuntimeValue::Object { .. } => "Object".to_string(),
            RuntimeValue::Lambda { .. } => "Lambda".to_string(),
            RuntimeValue::NativeFunction(_) => "NativeFunction".to_string(),
            RuntimeValue::None => "None".to_string(),
            RuntimeValue::Context => "Context".to_string(),
        }
    }
}

pub trait Evaluable {
    type Output;
    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output;
}

pub struct FabulistInterpreter;
