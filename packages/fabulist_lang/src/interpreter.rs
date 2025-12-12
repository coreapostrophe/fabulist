use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{dfn::models::ParameterBodyDfn, stmt::models::BlockStmt},
    environment::Environment,
};

pub enum RuntimeValue {
    Number(f32),
    Boolean(bool),
    String(String),
    None,
    Lambda {
        parameters: ParameterBodyDfn,
        body: BlockStmt,
        closure: Rc<RefCell<Environment>>,
    },
}

pub trait Evaluable {
    type Output;
    fn evaluate(&self, environment: &Rc<RefCell<Environment>>) -> Self::Output;
}

pub struct FabulistInterpreter;
