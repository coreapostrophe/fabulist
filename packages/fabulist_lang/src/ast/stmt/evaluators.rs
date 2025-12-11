use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::stmt::models::BlockStmt, environment::Environment, error::RuntimeError,
    interpreter::Evaluable,
};

impl Evaluable for BlockStmt {
    type Output = Result<(), RuntimeError>;

    fn evaluate(&self, _environment: &Rc<RefCell<Environment>>) -> Self::Output {
        todo!()
    }
}
