use std::ops::Add;

use pest::Span;

use crate::parser::Rule;

#[derive(Debug, Clone, Default)]
pub struct OwnedSpan {
    pub input: String,
    pub start: usize,
    pub end: usize,
}

impl From<Span<'_>> for OwnedSpan {
    fn from(value: Span<'_>) -> Self {
        OwnedSpan {
            input: value.get_input().to_string(),
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
    #[error("Invalid identifier in  lambda parameters.")]
    LambdaParameters(OwnedSpan),
    #[error("Unary operator `negation` can only be applied to numbers.")]
    UnaryNegationNonNumber(OwnedSpan),
    #[error("Unary operator `not` can only be applied to booleans.")]
    UnaryNotNonBoolean(OwnedSpan),
    #[error("Cannot call intrinsic number method on non-number value.")]
    InvalidNumberIntrinsicCall(OwnedSpan),
    #[error("Intrinsic methods can only be called on valid structures.")]
    InvalidIntrinsicCall(OwnedSpan),
    #[error("Missing intrinsic method call implementation.")]
    MissingIntrinsicCall(OwnedSpan),
    #[error("Attempted to call a non-callable value.")]
    CallNonCallable(OwnedSpan),
}
