//! Intrinsic helpers available to runtime values.
use std::{cell::RefCell, rc::Rc};

use crate::{
    error::{OwnedSpan, RuntimeError},
    interpreter::environment::Environment,
    interpreter::runtime_value::RuntimeValue,
};

/// Intrinsics for numeric runtime values.
pub(crate) struct NumberIntrinsics;

impl NumberIntrinsics {
    /// Converts a single numeric argument into its string representation.
    ///
    /// Returns a [`RuntimeError::InvalidArgumentsCount`] when called with the wrong
    /// arity or [`RuntimeError::TypeMismatch`] if the argument is not a number.
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

    /// Adds number intrinsics as a child environment and returns it.
    ///
    /// The returned environment currently exposes [`NumberIntrinsics::to_string`]
    /// under the symbol `to_string`.
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

/// Intrinsics for boolean runtime values.
pub(crate) struct BooleanIntrinsics;

impl BooleanIntrinsics {
    /// Attaches a fresh child environment for boolean intrinsics.
    ///
    /// Placeholder for future helpers.
    pub fn inject_intrinsics(environment: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Environment::add_empty_child(environment)
    }
}

/// Intrinsics for string runtime values.
pub(crate) struct StringIntrinsics;

impl StringIntrinsics {
    /// Attaches a fresh child environment for string intrinsics.
    ///
    /// Placeholder for future helpers.
    pub fn inject_intrinsics(environment: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Environment::add_empty_child(environment)
    }
}

/// Intrinsics for object runtime values.
pub(crate) struct ObjectIntrinsics;

impl ObjectIntrinsics {
    /// Attaches a fresh child environment for object intrinsics.
    ///
    /// Placeholder for future helpers.
    pub fn inject_intrinsics(environment: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Environment::add_empty_child(environment)
    }
}
