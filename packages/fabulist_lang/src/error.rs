//! Error types used by the parser and interpreter.
use std::ops::Add;

use pest::Span;

use crate::{interpreter::environment::EnvironmentError, parser::Rule};

/// Span owned by the AST for error reporting.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OwnedSpan {
    /// Input slice of the source string isolated to a symbol.
    pub input: String,
    /// Starting byte offset (inclusive).
    pub start: usize,
    /// Ending byte offset (exclusive).
    pub end: usize,
}

impl From<Span<'_>> for OwnedSpan {
    fn from(value: Span<'_>) -> Self {
        OwnedSpan {
            input: value.as_str().to_string(),
            start: value.start(),
            end: value.end(),
        }
    }
}

impl Add for OwnedSpan {
    type Output = OwnedSpan;
    fn add(self, rhs: Self) -> Self::Output {
        OwnedSpan {
            input: self.input,
            start: self.start,
            end: rhs.end,
        }
    }
}

/// Type alias for parsing errors.
pub type PestParsingError = pest::error::Error<Rule>;

/// Adapter for constructing custom pest errors from owned spans.
pub struct ParsingError;

impl ParsingError {
    /// Creates a `pest::error::Error` from an [`OwnedSpan`] and a message.
    ///
    /// [`OwnedSpan`]: crate::error::OwnedSpan
    pub fn map_custom_error(
        span: OwnedSpan,
        message: impl Into<String>,
    ) -> pest::error::Error<Rule> {
        let span = Span::new(&span.input, span.start, span.end)
            .expect("`OwnedSpan` indices are out of bounds.");
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: message.into(),
            },
            span,
        )
    }
}

#[derive(thiserror::Error, Debug)]
/// Errors produced at runtime by the interpreter.
pub enum RuntimeError {
    /// Identifier failed validation before lookup.
    #[error("Invalid identifier.")]
    InvalidIdentifier(OwnedSpan),
    /// Identifier was not found in any environment.
    #[error("Identifier does not exist in environment.")]
    IdentifierDoesNotExist(OwnedSpan),
    /// The unary negation operator was applied to a non-number.
    #[error("Unary operator `negation` can only be applied to numbers.")]
    UnaryNegationNonNumber(OwnedSpan),
    /// The unary logical-not operator was applied to a non-boolean.
    #[error("Unary operator `not` can only be applied to booleans.")]
    UnaryNotNonBoolean(OwnedSpan),
    /// A call attempted to invoke a value that is not callable.
    #[error("Value is not callable.")]
    CallNonCallable(OwnedSpan),
    /// Intrinsic function raised an error.
    #[error("Error when calling intrinsic function: `{0}`.")]
    IntrinsicFunctionError(String, OwnedSpan),
    /// The wrong number of arguments was provided to a callable.
    #[error("Argument type mismatch. Expected `{expected}`, got `{got}`.")]
    InvalidArgumentsCount {
        /// Expected arity of the callee.
        expected: usize,
        /// Actual number of arguments supplied.
        got: usize,
        /// Span covering the call site.
        span: OwnedSpan,
    },
    /// A value's runtime type did not match the expected one.
    #[error("Argument type mismatch. Expected `{expected}`, got `{got}`.")]
    TypeMismatch {
        /// Type the interpreter was expecting.
        expected: String,
        /// Type encountered during evaluation.
        got: String,
        /// Span covering the mismatch.
        span: OwnedSpan,
    },
    /// A value could not be coerced to boolean.
    #[error("Cannot cast value to boolean.")]
    CannotCastToBoolean(OwnedSpan),
    /// A value could not be coerced to number.
    #[error("Cannot cast value to number.")]
    CannotCastToNumber(OwnedSpan),
    /// Parsing a string to a number failed.
    #[error("Cannot parse string `{value}` to number.")]
    CannotParseStringToNumber {
        /// String that failed to parse.
        value: String,
        /// Span covering the literal.
        span: OwnedSpan,
    },
    /// Multiplication could not be performed for the given operands.
    #[error("Invalid multiplication operation: {message}")]
    InvalidMultiplication {
        /// Explanation of why the operation failed.
        message: String,
        /// Span covering the invalid operation.
        span: OwnedSpan,
    },
    /// Division could not be performed for the given operands.
    #[error("Invalid division operation: {message}")]
    InvalidDivision {
        /// Explanation of why the operation failed.
        message: String,
        /// Span covering the invalid operation.
        span: OwnedSpan,
    },
    /// Assignment target was not an identifier.
    #[error("Invalid assignment to non-identifier.")]
    AssignmentToNonIdentifier(OwnedSpan),
    /// Invalid or unsupported member access was attempted.
    #[error("Invalid memory access.")]
    InvalidMemoryAccess(OwnedSpan),
    /// Errors originating from the environment.
    #[error("Environment error: {0}")]
    EnvironmentError(#[from] EnvironmentError),
}
