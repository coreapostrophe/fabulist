use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::stmt::models::BlockStmt, context::Context, environment::Environment, error::RuntimeError,
    interpreter::Evaluable,
};

impl Evaluable for BlockStmt {
    type Output = Result<(), RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        todo!()
    }
}
