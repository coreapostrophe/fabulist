//! Error types used by the parser and interpreter.

use crate::interpreter::environment::EnvironmentError;

#[derive(thiserror::Error, Debug)]
/// Errors produced at runtime by the interpreter.
pub enum RuntimeError {
    /// Identifier failed validation before lookup.
    #[error("Invalid identifier.")]
    InvalidIdentifier(crate::parser::error::SpanSlice),
    /// Identifier was not found in any environment.
    #[error("Identifier does not exist in environment.")]
    IdentifierDoesNotExist(crate::parser::error::SpanSlice),
    /// The unary negation operator was applied to a non-number.
    #[error("Unary operator `negation` can only be applied to numbers.")]
    UnaryNegationNonNumber(crate::parser::error::SpanSlice),
    /// The unary logical-not operator was applied to a non-boolean.
    #[error("Unary operator `not` can only be applied to booleans.")]
    UnaryNotNonBoolean(crate::parser::error::SpanSlice),
    /// A call attempted to invoke a value that is not callable.
    #[error("Value is not callable.")]
    CallNonCallable(crate::parser::error::SpanSlice),
    /// Intrinsic function raised an error.
    #[error("Error when calling intrinsic function: `{0}`.")]
    IntrinsicFunctionError(String, crate::parser::error::SpanSlice),
    /// The wrong number of arguments was provided to a callable.
    #[error("Argument type mismatch. Expected `{expected}`, got `{got}`.")]
    InvalidArgumentsCount {
        /// Expected arity of the callee.
        expected: usize,
        /// Actual number of arguments supplied.
        got: usize,
        /// Span covering the call site.
        span: crate::parser::error::SpanSlice,
    },
    /// A value's runtime type did not match the expected one.
    #[error("Argument type mismatch. Expected `{expected}`, got `{got}`.")]
    TypeMismatch {
        /// Type the interpreter was expecting.
        expected: String,
        /// Type encountered during evaluation.
        got: String,
        /// Span covering the mismatch.
        span: crate::parser::error::SpanSlice,
    },
    /// A value could not be coerced to boolean.
    #[error("Cannot cast value to boolean.")]
    CannotCastToBoolean(crate::parser::error::SpanSlice),
    /// A value could not be coerced to number.
    #[error("Cannot cast value to number.")]
    CannotCastToNumber(crate::parser::error::SpanSlice),
    /// Parsing a string to a number failed.
    #[error("Cannot parse string `{value}` to number.")]
    CannotParseStringToNumber {
        /// String that failed to parse.
        value: String,
        /// Span covering the literal.
        span: crate::parser::error::SpanSlice,
    },
    /// Multiplication could not be performed for the given operands.
    #[error("Invalid multiplication operation: {message}")]
    InvalidMultiplication {
        /// Explanation of why the operation failed.
        message: String,
        /// Span covering the invalid operation.
        span: crate::parser::error::SpanSlice,
    },
    /// Division could not be performed for the given operands.
    #[error("Invalid division operation: {message}")]
    InvalidDivision {
        /// Explanation of why the operation failed.
        message: String,
        /// Span covering the invalid operation.
        span: crate::parser::error::SpanSlice,
    },
    /// Assignment target was not an identifier.
    #[error("Invalid assignment to non-identifier.")]
    AssignmentToNonIdentifier(crate::parser::error::SpanSlice),
    /// Invalid or unsupported member access was attempted.
    #[error("Invalid memory access.")]
    InvalidMemoryAccess(crate::parser::error::SpanSlice),
    /// Errors originating from the environment.
    #[error("Environment error: {0}")]
    EnvironmentError(#[from] EnvironmentError),
}
