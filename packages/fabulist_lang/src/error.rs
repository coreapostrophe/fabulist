use std::ops::Add;

use pest::Span;

use crate::parser::Rule;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OwnedSpan {
    pub input: String,
    pub start: usize,
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

pub struct ParsingError;

impl ParsingError {
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
pub enum RuntimeError {
    #[error("Invalid identifier.")]
    InvalidIdentifier(OwnedSpan),
    #[error("Unary operator `negation` can only be applied to numbers.")]
    UnaryNegationNonNumber(OwnedSpan),
    #[error("Unary operator `not` can only be applied to booleans.")]
    UnaryNotNonBoolean(OwnedSpan),
    #[error("Value is not callable.")]
    CallNonCallable(OwnedSpan),
    #[error("Error when calling intrinsic function: `{0}`.")]
    IntrinsicFunctionError(String, OwnedSpan),
    #[error("Argument type mismatch. Expected `{expected}`, got `{got}`.")]
    InvalidArgumentsCount {
        expected: usize,
        got: usize,
        span: OwnedSpan,
    },
    #[error("Argument type mismatch. Expected `{expected}`, got `{got}`.")]
    TypeMismatch {
        expected: String,
        got: String,
        span: OwnedSpan,
    },
    #[error("Cannot cast value to boolean.")]
    CannotCastToBoolean(OwnedSpan),
    #[error("Cannot cast value to number.")]
    CannotCastToNumber(OwnedSpan),
    #[error("Cannot parse string `{value}` to number.")]
    CannotParseStringToNumber { value: String, span: OwnedSpan },
    #[error("Invalid multiplication operation: {message}")]
    InvalidMultiplication { message: String, span: OwnedSpan },
    #[error("Invalid division operation: {message}")]
    InvalidDivision { message: String, span: OwnedSpan },
    #[error("Invalid assignment to non-identifier.")]
    AssignmentToNonIdentifier(OwnedSpan),
}
