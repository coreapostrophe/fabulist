use std::{cell::RefCell, rc::Rc};

use fabulist_core::story::context::Context;

use crate::environment::Environment;

pub trait Evaluable {
    type Output;
    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output;
}

pub struct FabulistInterpreter;
