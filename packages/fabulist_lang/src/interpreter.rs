use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{dfn::models::ParameterBodyDfn, stmt::models::BlockStmt},
    context::Context,
    environment::Environment,
    error::{OwnedSpan, RuntimeError},
};

#[derive(Clone, Debug)]
pub enum RuntimeValue {
    Number(f32),
    Boolean(bool),
    String(String),
    Identifier(String),
    Object(HashMap<String, RuntimeValue>),
    Lambda {
        parameters: ParameterBodyDfn,
        body: BlockStmt,
        closure: Rc<RefCell<Environment>>,
    },
    NativeFunction(fn(Vec<RuntimeValue>, OwnedSpan) -> Result<RuntimeValue, RuntimeError>),
    None,
    Context,
}

impl RuntimeValue {
    pub fn type_name(&self) -> String {
        match self {
            RuntimeValue::Number(_) => "Number".to_string(),
            RuntimeValue::Boolean(_) => "Boolean".to_string(),
            RuntimeValue::String(_) => "String".to_string(),
            RuntimeValue::Identifier(_) => "Identifier".to_string(),
            RuntimeValue::Object(_) => "Object".to_string(),
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
