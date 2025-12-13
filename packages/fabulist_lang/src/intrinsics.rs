use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    error::{OwnedSpan, RuntimeError},
    interpreter::RuntimeValue,
};

pub struct NumberIntrinsics;

impl NumberIntrinsics {
    pub fn to_string(
        args: Vec<RuntimeValue>,
        span: OwnedSpan,
    ) -> Result<RuntimeValue, RuntimeError> {
        if args.len() != 1 {
            return Err(RuntimeError::InvalidArgumentsCount {
                expected: 1,
                got: args.len(),
                span,
            });
        }

        match &args[0] {
            RuntimeValue::Number(n) => Ok(RuntimeValue::String(n.to_string())),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
                got: args[0].type_name(),
                span,
            }),
        }
    }

    pub fn inject_intrinsics(environment: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        let closure_environment = Environment::add_empty_child(environment);

        Environment::insert(
            &closure_environment,
            "to_string".to_string(),
            RuntimeValue::NativeFunction(Self::to_string),
        );

        closure_environment
    }
}
