use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        dfn::models::{ObjectDfn, ParameterBodyDfn},
        expr::models::{Expr, IdentifierPrimitive},
    },
    environment::Environment,
    error::RuntimeError,
    interpreter::Evaluable,
};

impl Evaluable for ObjectDfn {
    type Output = Result<HashMap<String, Expr>, RuntimeError>;

    fn evaluate(&self, _environment: &Rc<RefCell<Environment>>) -> Self::Output {
        Ok(self.object.clone())
    }
}

impl Evaluable for ParameterBodyDfn {
    type Output = Result<Option<Vec<IdentifierPrimitive>>, RuntimeError>;

    fn evaluate(&self, _environment: &Rc<RefCell<Environment>>) -> Self::Output {
        Ok(self.parameters.clone())
    }
}
