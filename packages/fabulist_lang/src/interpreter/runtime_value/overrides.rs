//! RuntimeValue helpers and arithmetic implementations.
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

use crate::interpreter::{error::RuntimeError, runtime_value::RuntimeValue};
use crate::parser::error::SpanSlice;

impl RuntimeValue {
    /// Returns the span associated with the value.
    pub fn span(&self) -> SpanSlice {
        match self {
            RuntimeValue::Number { span, .. }
            | RuntimeValue::Boolean { span, .. }
            | RuntimeValue::String { span, .. }
            | RuntimeValue::Identifier { span, .. }
            | RuntimeValue::Object { span, .. }
            | RuntimeValue::Lambda { span, .. }
            | RuntimeValue::None { span, .. }
            | RuntimeValue::Context { span, .. }
            | RuntimeValue::Module { span, .. }
            | RuntimeValue::Path { span, .. } => span.clone(),
            RuntimeValue::NativeFunction(_) => unreachable!("Native functions do not have spans."),
        }
    }

    /// Attempts to coerce the value to a boolean, returning a runtime error when coercion fails.
    pub(crate) fn to_bool(&self) -> Result<bool, RuntimeError> {
        match self {
            RuntimeValue::Boolean { value, .. } => Ok(*value),
            RuntimeValue::Number { value, .. } => Ok(*value != 0.0),
            RuntimeValue::String { value, .. } => Ok(!value.is_empty()),
            RuntimeValue::None { .. } => Ok(false),
            other => Err(RuntimeError::CannotCastToBoolean(other.span())),
        }
    }

    /// Attempts to coerce the value to a number, returning a runtime error when coercion fails.
    pub(crate) fn to_num(&self) -> Result<f32, RuntimeError> {
        match self {
            RuntimeValue::Number { value, .. } => Ok(*value),
            RuntimeValue::Boolean { value, .. } => Ok(if *value { 1.0 } else { 0.0 }),
            RuntimeValue::None { .. } => Ok(0.0),
            RuntimeValue::String { span, value } => {
                value
                    .parse::<f32>()
                    .map_err(|_| RuntimeError::CannotParseStringToNumber {
                        value: value.clone(),
                        span: span.clone(),
                    })
            }
            other => Err(RuntimeError::CannotCastToNumber(other.span())),
        }
    }
}

impl Add for RuntimeValue {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            RuntimeValue::String {
                value: addend1_value,
                span: addend1_span,
            } => match rhs {
                RuntimeValue::Number {
                    value: addend2_value,
                    span: addend2_span,
                } => Ok(RuntimeValue::String {
                    span: addend1_span.clone() + addend2_span,
                    value: format!("{}{}", addend1_value, addend2_value),
                }),
                RuntimeValue::String {
                    value: addend2_value,
                    span: addend2_span,
                } => Ok(RuntimeValue::String {
                    span: addend1_span.clone() + addend2_span,
                    value: format!("{}{}", addend1_value, addend2_value),
                }),
                RuntimeValue::Boolean {
                    value: addend2_value,
                    span: addend2_span,
                } => Ok(RuntimeValue::String {
                    span: addend1_span.clone() + addend2_span,
                    value: format!("{}{}", addend1_value, addend2_value),
                }),
                RuntimeValue::None { span: addend2_span } => Ok(RuntimeValue::String {
                    span: addend1_span.clone() + addend2_span,
                    value: addend1_value,
                }),
                other => Err(RuntimeError::TypeMismatch {
                    expected: "Number | Boolean | String | None".to_string(),
                    got: other.type_name(),
                    span: other.span(),
                }),
            },
            _ => Ok(RuntimeValue::Number {
                span: self.span() + rhs.span(),
                value: self.to_num()? + rhs.to_num()?,
            }),
        }
    }
}

impl Mul for RuntimeValue {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            RuntimeValue::None { span } => Err(RuntimeError::InvalidMultiplication {
                message: "Unable to multiply `none` literal".to_string(),
                span,
            }),
            _ => match rhs {
                RuntimeValue::None { span } => Err(RuntimeError::InvalidMultiplication {
                    message: "Unable to multiply `none` literal".to_string(),
                    span,
                }),
                _ => Ok(RuntimeValue::Number {
                    span: self.span() + rhs.span(),
                    value: self.to_num()? * rhs.to_num()?,
                }),
            },
        }
    }
}

impl Sub for RuntimeValue {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        Ok(RuntimeValue::Number {
            span: self.span() + rhs.span(),
            value: self.to_num()? - rhs.to_num()?,
        })
    }
}

impl Div for RuntimeValue {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            RuntimeValue::None { span } => Err(RuntimeError::InvalidDivision {
                message: "Unable to divide `none` literal".to_string(),
                span,
            }),
            _ => match rhs {
                RuntimeValue::None { span } => Err(RuntimeError::InvalidDivision {
                    message: "Unable to divide `none` literal".to_string(),
                    span,
                }),
                _ => Ok(RuntimeValue::Number {
                    span: self.span() + rhs.span(),
                    value: self.to_num()? / rhs.to_num()?,
                }),
            },
        }
    }
}

impl PartialEq for RuntimeValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RuntimeValue::Number { value, .. } => match other {
                RuntimeValue::Number {
                    value: other_value, ..
                } => value == other_value,
                _ => false,
            },
            RuntimeValue::Boolean { value, .. } => match other {
                RuntimeValue::Boolean {
                    value: other_value, ..
                } => value == other_value,
                _ => false,
            },
            RuntimeValue::String { value, .. } => match other {
                RuntimeValue::String {
                    value: other_value, ..
                } => value == other_value,
                _ => false,
            },
            RuntimeValue::None { .. } => matches!(other, RuntimeValue::None { .. }),
            _ => false,
        }
    }
}

impl PartialOrd for RuntimeValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            RuntimeValue::Number { value, .. } => match other {
                RuntimeValue::Number {
                    value: other_value, ..
                } => value.partial_cmp(other_value),
                _ => None,
            },
            RuntimeValue::Boolean { value, .. } => match other {
                RuntimeValue::Boolean {
                    value: other_value, ..
                } => value.partial_cmp(other_value),
                _ => None,
            },
            RuntimeValue::String { value, .. } => match other {
                RuntimeValue::String {
                    value: other_value, ..
                } => value.partial_cmp(other_value),
                _ => None,
            },
            RuntimeValue::None { .. } => None,
            _ => None,
        }
    }
}

#[cfg(test)]
mod expr_overrides_tests {
    use crate::{interpreter::runtime_value::RuntimeValue, parser::error::SpanSlice};

    impl From<f32> for RuntimeValue {
        fn from(value: f32) -> Self {
            RuntimeValue::Number {
                span: SpanSlice::default(),
                value,
            }
        }
    }

    impl From<bool> for RuntimeValue {
        fn from(value: bool) -> Self {
            RuntimeValue::Boolean {
                span: SpanSlice::default(),
                value,
            }
        }
    }

    impl From<&str> for RuntimeValue {
        fn from(value: &str) -> Self {
            RuntimeValue::String {
                span: SpanSlice::default(),
                value: value.to_string(),
            }
        }
    }

    impl From<()> for RuntimeValue {
        fn from(_: ()) -> Self {
            RuntimeValue::None {
                span: SpanSlice::default(),
            }
        }
    }

    #[test]
    fn runtime_value_to_num_works() {
        assert_eq!(RuntimeValue::from(5.0).to_num().unwrap(), 5.0);
        assert_eq!(RuntimeValue::from(true).to_num().unwrap(), 1.0);
        assert_eq!(RuntimeValue::from(()).to_num().unwrap(), 0.0);
        assert_eq!(RuntimeValue::from("10").to_num().unwrap(), 10.0);

        assert!(RuntimeValue::from("hello").to_num().is_err());
        assert_eq!(
            RuntimeValue::from("hello")
                .to_num()
                .err()
                .unwrap()
                .to_string(),
            "Cannot parse string `hello` to number."
        );
    }

    #[test]
    fn add_runtime_value_works() {
        // number + <value>
        let result = RuntimeValue::from(5.0) + RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 10.0);

        let result = RuntimeValue::from(5.0) + RuntimeValue::from(10.0);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 15.0);

        let result = RuntimeValue::from(5.0) + RuntimeValue::from(0.0);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 5.0);

        let result = RuntimeValue::from(5.0) + RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 6.0);

        // boolean + <value>
        let result = RuntimeValue::from(true) + RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 6.0);

        let result = RuntimeValue::from(true) + RuntimeValue::from(0.0);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 1.0);

        let result = RuntimeValue::from(true) + RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 2.0);

        let result = RuntimeValue::from(true) + RuntimeValue::from(10.0);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 11.0);

        // string + <value>
        let result = RuntimeValue::from("10") + RuntimeValue::from(5.0);
        let RuntimeValue::String { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a String runtime value");
        };
        assert_eq!(value, "105".to_string());

        let result = RuntimeValue::from("10") + RuntimeValue::from(true);
        let RuntimeValue::String { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a String runtime value");
        };
        assert_eq!(value, "10true".to_string());

        let result = RuntimeValue::from("10") + RuntimeValue::from(());
        let RuntimeValue::String { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a String runtime value");
        };
        assert_eq!(value, "10".to_string());

        let result = RuntimeValue::from("10") + RuntimeValue::from("10");
        let RuntimeValue::String { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a String runtime value");
        };
        assert_eq!(value, "1010".to_string());

        // none + <value>
        let result = RuntimeValue::from(()) + RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 5.0);

        let result = RuntimeValue::from(()) + RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 1.0);

        let result = RuntimeValue::from(()) + RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 10.0);

        let result = RuntimeValue::from(()) + RuntimeValue::from(());
        let RuntimeValue::Number { value, .. } = result.expect("Add failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 0.0);
    }

    #[test]
    fn mul_runtime_value_works() {
        // number * <value>
        let result = RuntimeValue::from(5.0) * RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 25.0);

        let result = RuntimeValue::from(5.0) * RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 5.0);

        let result = RuntimeValue::from(5.0) * RuntimeValue::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid multiplication operation: Unable to multiply `none` literal"
        );

        let result = RuntimeValue::from(5.0) * RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 50.0);

        // boolean * <value>
        let result = RuntimeValue::from(true) * RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 5.0);

        let result = RuntimeValue::from(true) * RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 1.0);

        let result = RuntimeValue::from(true) * RuntimeValue::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid multiplication operation: Unable to multiply `none` literal"
        );

        let result = RuntimeValue::from(true) * RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 10.0);

        // string * <value>
        let result = RuntimeValue::from("10") * RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 50.0);

        let result = RuntimeValue::from("10") * RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 10.0);

        let result = RuntimeValue::from("10") * RuntimeValue::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid multiplication operation: Unable to multiply `none` literal"
        );

        let result = RuntimeValue::from("10") * RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 100.0);

        // none * <value>
        let result = RuntimeValue::from(()) * RuntimeValue::from(5.0);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid multiplication operation: Unable to multiply `none` literal"
        );

        let result = RuntimeValue::from(()) * RuntimeValue::from(true);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid multiplication operation: Unable to multiply `none` literal"
        );

        let result = RuntimeValue::from(()) * RuntimeValue::from("10");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid multiplication operation: Unable to multiply `none` literal"
        );

        let result = RuntimeValue::from(()) * RuntimeValue::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid multiplication operation: Unable to multiply `none` literal"
        );
    }

    #[test]
    fn div_runtime_value_works() {
        // number / <value>
        let result = RuntimeValue::from(5.0) / RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Div failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 1.0);

        let result = RuntimeValue::from(5.0) / RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Div failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 5.0);

        let result = RuntimeValue::from(5.0) / RuntimeValue::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid division operation: Unable to divide `none` literal"
        );

        let result = RuntimeValue::from(5.0) / RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Div failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 0.5);

        // boolean / <value>
        let result = RuntimeValue::from(true) / RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Div failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 0.2);

        let result = RuntimeValue::from(true) / RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Div failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 1.0);

        let result = RuntimeValue::from(true) / RuntimeValue::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid division operation: Unable to divide `none` literal"
        );

        let result = RuntimeValue::from(true) / RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Div failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 0.1);

        // string / <value>
        let result = RuntimeValue::from("10") / RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Div failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 2.0);

        let result = RuntimeValue::from("10") / RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Div failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 10.0);

        let result = RuntimeValue::from("10") / RuntimeValue::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid division operation: Unable to divide `none` literal"
        );

        let result = RuntimeValue::from("10") / RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Div failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 1.0);

        // none / <value>
        let result = RuntimeValue::from(()) / RuntimeValue::from(5.0);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid division operation: Unable to divide `none` literal"
        );

        let result = RuntimeValue::from(()) / RuntimeValue::from(true);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid division operation: Unable to divide `none` literal"
        );

        let result = RuntimeValue::from(()) / RuntimeValue::from("10");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid division operation: Unable to divide `none` literal"
        );

        let result = RuntimeValue::from(()) / RuntimeValue::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid division operation: Unable to divide `none` literal"
        );
    }

    #[test]
    fn sub_runtime_value_works() {
        // number - <value>
        let result = RuntimeValue::from(5.0) - RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 0.0);

        let result = RuntimeValue::from(5.0) - RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 4.0);

        let result = RuntimeValue::from(5.0) - RuntimeValue::from(());
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 5.0);

        let result = RuntimeValue::from(5.0) - RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, -5.0);

        // boolean - <value>
        let result = RuntimeValue::from(true) - RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, -4.0);

        let result = RuntimeValue::from(true) - RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 0.0);

        let result = RuntimeValue::from(true) - RuntimeValue::from(());
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 1.0);

        let result = RuntimeValue::from(true) - RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, -9.0);

        // string - <value>
        let result = RuntimeValue::from("10") - RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 5.0);

        let result = RuntimeValue::from("10") - RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 9.0);

        let result = RuntimeValue::from("10") - RuntimeValue::from(());
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 10.0);

        let result = RuntimeValue::from("10") - RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 0.0);

        // none - <value>
        let result = RuntimeValue::from(()) - RuntimeValue::from(5.0);
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, -5.0);

        let result = RuntimeValue::from(()) - RuntimeValue::from(true);
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, -1.0);

        let result = RuntimeValue::from(()) - RuntimeValue::from("10");
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, -10.0);

        let result = RuntimeValue::from(()) - RuntimeValue::from(());
        let RuntimeValue::Number { value, .. } = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a Number runtime value");
        };
        assert_eq!(value, 0.0);
    }

    #[test]
    fn eq_runtime_value_works() {
        assert_eq!(RuntimeValue::from(5.0), RuntimeValue::from(5.0));
        assert_ne!(RuntimeValue::from(5.0), RuntimeValue::from(10.0));
        assert_eq!(RuntimeValue::from(true), RuntimeValue::from(true));
        assert_ne!(RuntimeValue::from(true), RuntimeValue::from(false));
        assert_eq!(RuntimeValue::from("hello"), RuntimeValue::from("hello"));
        assert_ne!(RuntimeValue::from("hello"), RuntimeValue::from("world"));
        assert_eq!(RuntimeValue::from(()), RuntimeValue::from(()));
        assert_ne!(RuntimeValue::from(()), RuntimeValue::from(5.0));
    }

    #[test]
    fn ord_runtime_value_works() {
        assert!(RuntimeValue::from(5.0) < RuntimeValue::from(10.0));
        assert!(RuntimeValue::from(10.0) > RuntimeValue::from(5.0));
        assert!(RuntimeValue::from(true) > RuntimeValue::from(false));
        assert!(RuntimeValue::from(false) < RuntimeValue::from(true));
        assert!(RuntimeValue::from("apple") < RuntimeValue::from("banana"));
        assert!(RuntimeValue::from("banana") > RuntimeValue::from("apple"));
    }
}
