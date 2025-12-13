use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::stmt::models::BlockStmt,
    context::Context,
    environment::Environment,
    error::RuntimeError,
    interpreter::{runtime_value::RuntimeValue, Evaluable},
};

impl Evaluable for BlockStmt {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        todo!()
    }
}
