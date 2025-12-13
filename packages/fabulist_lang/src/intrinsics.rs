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

    pub fn get_intrinsics_env() -> Rc<RefCell<Environment>> {
        let intrinsics_environment = Environment::new();

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
    pub fn get_intrinsics_env() -> Rc<RefCell<Environment>> {
        todo!()
    }
}

pub struct StringIntrinsics;

impl StringIntrinsics {
    pub fn get_intrinsics_env() -> Rc<RefCell<Environment>> {
        todo!()
    }
}

pub struct ObjectIntrinsics;

impl ObjectIntrinsics {
    pub fn get_intrinsics_env() -> Rc<RefCell<Environment>> {
        todo!()
    }
}
