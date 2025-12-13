use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        dfn::models::{ArgumentBodyDfn, ObjectDfn, ParameterBodyDfn},
        expr::models::IdentifierPrimitive,
    },
    context::Context,
    environment::Environment,
    error::RuntimeError,
    interpreter::{runtime_value::RuntimeValue, Evaluable},
};

impl Evaluable for ObjectDfn {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        self.object
            .clone()
            .into_iter()
            .map(|(key, expr)| {
                let value = expr.evaluate(environment, context)?;
                Ok((key.clone(), value))
            })
            .collect::<Result<HashMap<String, RuntimeValue>, RuntimeError>>()
            .map(|properties| RuntimeValue::Object {
                properties,
                span: self.span.clone(),
            })
    }
}

impl Evaluable for ParameterBodyDfn {
    type Output = Result<Option<Vec<IdentifierPrimitive>>, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(self.parameters.clone())
    }
}

impl Evaluable for ArgumentBodyDfn {
    type Output = Result<Option<Vec<RuntimeValue>>, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        match &self.arguments {
            Some(args) => args
                .iter()
                .map(|arg| arg.evaluate(environment, context))
                .collect::<Result<Vec<RuntimeValue>, RuntimeError>>()
                .map(Some),
            None => Ok(None),
        }
    }
}
