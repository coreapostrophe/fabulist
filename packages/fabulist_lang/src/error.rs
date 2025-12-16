//! Error types used by the parser and interpreter.
use std::ops::Add;

use pest::Span;

use crate::{interpreter::environment::EnvironmentError, parser::Rule};

/// Slice of the source input with the original byte offsets.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpanSlice {
    /// Input slice of the source string.
    pub slice: String,
    /// Starting byte offset of the input string (inclusive).
    pub input_start: usize,
    /// Ending byte offset of the input string (exclusive).
    pub input_end: usize,
}

impl From<Span<'_>> for SpanSlice {
    fn from(value: Span<'_>) -> Self {
        let input = value.get_input().to_string();
        let slice = &input[value.start()..value.end()];
        let trimmed_slice = slice.trim().to_string();

        SpanSlice {
            slice: trimmed_slice,
            input_start: value.start(),
            input_end: value.end(),
        }
    }
}

impl Add for SpanSlice {
    type Output = SpanSlice;
    fn add(self, rhs: Self) -> Self::Output {
        SpanSlice {
            slice: self.slice,
            input_start: self.input_start,
            input_end: rhs.input_end,
        }
    }
}

/// Type alias for parsing errors.
pub type PestParsingError = pest::error::Error<Rule>;

/// Adapter for constructing custom pest errors from owned spans.
pub struct ParsingError;

impl ParsingError {
    /// Creates a `pest::error::Error` from an [`SpanSlice`] and a message.
    ///
    /// [`SpanSlice`]: crate::error::SpanSlice
    pub fn map_custom_error(
        span: SpanSlice,
        message: impl Into<String>,
    ) -> pest::error::Error<Rule> {
        let span = Span::new(&span.slice, span.input_start, span.input_end)
            .expect("`SpanSlice` indices are out of bounds.");
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
