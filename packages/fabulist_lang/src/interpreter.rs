use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{dfn::models::ParameterBodyDfn, stmt::models::BlockStmt},
    context::Context,
    environment::Environment,
    error::OwnedSpan,
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
    NativeFunction(fn(Vec<RuntimeValue>, OwnedSpan) -> RuntimeValue),
    None,
    Context,
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
