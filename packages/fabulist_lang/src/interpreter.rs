use std::{cell::RefCell, rc::Rc};

use crate::environment::Environment;

pub enum RuntimeValue {
    Number(f32),
    Boolean(bool),
    String(String),
    None,
}

pub trait Evaluable {
    type Output;
    fn evaluate(&self, environment: &Rc<RefCell<Environment>>) -> Self::Output;
}

pub struct FabulistInterpreter;
