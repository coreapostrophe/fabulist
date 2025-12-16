//! Error types used by the parser and interpreter.
use std::{fmt::Display, ops::Add};

use pest::iterators::Pair;

use crate::parser::Rule;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineCol {
    pub line: usize,
    pub column: usize,
}

impl Display for LineCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpanSlice {
    slice: String,
    line_col: LineCol,
    input_start: usize,
    input_end: usize,
}

pub trait ExtractSpanSlice {
    fn extract_span_slice(&self) -> SpanSlice;
}

impl ExtractSpanSlice for Pair<'_, Rule> {
    fn extract_span_slice(&self) -> SpanSlice {
        let span = self.as_span();
        let input = span.get_input().to_string();
        let slice = &input[span.start()..span.end()];
        let trimmed_slice = slice.trim().to_string();
        let (line, column) = span.start_pos().line_col();

        SpanSlice {
            slice: trimmed_slice,
            line_col: LineCol { line, column },
            input_start: span.start(),
            input_end: span.end(),
        }
    }
}

impl Add for SpanSlice {
    type Output = SpanSlice;

    fn add(self, other: SpanSlice) -> SpanSlice {
        let (first_span_slice, second_span_slice) = if self.input_start <= other.input_start {
            (self, other)
        } else {
            (other, self)
        };

        let combined_slice = format!("{}{}", first_span_slice.slice, second_span_slice.slice);

        SpanSlice {
            slice: combined_slice,
            line_col: first_span_slice.line_col.clone(),
            input_start: first_span_slice.input_start,
            input_end: second_span_slice.input_end,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error("Expected {expected:?}")]
    ExpectedSymbol {
        expected: String,
        span_slice: SpanSlice,
    },
    #[error("Invalid boolean literal.")]
    InvalidBooleanLiteral(SpanSlice),
    #[error("Unable to cast number literal.")]
    UnableToCastNumber(SpanSlice),
    #[error("Invalid primary expression.")]
    InvalidPrimaryExpression(SpanSlice),
    #[error("Invalid binary operator.")]
    InvalidBinaryOperator(SpanSlice),
    #[error("Invalid identifier primitive.")]
    InvalidIdentifierPrimitive(SpanSlice),
    #[error("Invalid primitive.")]
    InvalidPrimitive(SpanSlice),
    #[error("Invalid literal.")]
    InvalidLiteral(SpanSlice),
    #[error("Invalid statement.")]
    InvalidStatement(SpanSlice),
    #[error("Invalid expression.")]
    InvalidExpression(SpanSlice),
    #[error("Invalid definition.")]
    InvalidDefinition(SpanSlice),
    #[error("Invalid declaration.")]
    InvalidDeclaration(SpanSlice),
    #[error("Unable to parse story.")]
    UnableToParseStory(SpanSlice),
    #[error("Failed pest parsing: {0}")]
    PestParsing(#[from] Box<pest::error::Error<Rule>>),
}

impl ParserError {
    pub fn span_slice(&self) -> &SpanSlice {
        match self {
            ParserError::InvalidDeclaration(span_slice)
            | ParserError::UnableToParseStory(span_slice) => span_slice,
            ParserError::ExpectedSymbol { span_slice, .. } => span_slice,
            _ => panic!("No span slice available for this error type"),
        }
    }
}

pub type ParserResult<T> = Result<T, ParserError>;
