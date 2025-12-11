use std::{cell::RefCell, rc::Rc};

use fabulist_core::story::context::Context;

use crate::{
    ast::expr::models::{Literal, Unary},
    environment::Environment,
    interpreter::Evaluable,
    parser::Rule,
};

impl Evaluable for Unary {
    type Output = Result<Literal, pest::error::Error<Rule>>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        todo!()
    }
}
