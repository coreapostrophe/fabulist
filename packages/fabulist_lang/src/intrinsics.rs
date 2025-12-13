use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    error::{OwnedSpan, RuntimeError},
    interpreter::runtime_value::RuntimeValue,
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
            RuntimeValue::Number { value: n, .. } => Ok(RuntimeValue::String {
                value: n.to_string(),
                span: span.clone(),
            }),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
                got: args[0].type_name(),
                span,
            }),
        }
    }

    pub fn inject_intrinsics(environment: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        let intrinsics_environment = Environment::add_empty_child(environment);

        Environment::insert(
            &intrinsics_environment,
            "to_string".to_string(),
            RuntimeValue::NativeFunction(Self::to_string),
        );

        intrinsics_environment
    }
}

pub struct BooleanIntrinsics;

impl BooleanIntrinsics {
    pub fn inject_intrinsics(environment: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Environment::add_empty_child(environment)
    }
}

pub struct StringIntrinsics;

impl StringIntrinsics {
    pub fn inject_intrinsics(environment: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Environment::add_empty_child(environment)
    }
}

pub struct ObjectIntrinsics;

impl ObjectIntrinsics {
    pub fn inject_intrinsics(environment: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Environment::add_empty_child(environment)
    }
}
