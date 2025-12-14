use std::{cell::RefCell, rc::Rc};

use crate::{context::Context, environment::Environment};

pub mod runtime_value;

pub trait Evaluable {
    type Output;
    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output;
}

pub struct FabulistInterpreter;

impl FabulistInterpreter {
    pub fn evaluate<T: Evaluable>(
        node: &T,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> T::Output {
        node.evaluate(environment, context)
    }
}
