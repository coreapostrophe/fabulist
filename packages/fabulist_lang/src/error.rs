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
}
