//! Parser error types and utilities for the Fabulist language.

#![warn(missing_docs)]

use std::{fmt::Display, ops::Add};

use pest::iterators::Pair;

use crate::parser::Rule;

/// Represents a line and column position in the source code.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineCol {
    /// The line number (1-based).
    pub line: usize,
    /// The column number (1-based).
    pub column: usize,
}

impl Display for LineCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A slice of the source code along with its position information.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpanSlice {
    /// The actual slice of source code.
    pub slice: String,
    /// The line and column information.
    pub line_col: LineCol,
    /// The starting byte index in the original input.
    pub input_start: usize,
    /// The ending byte index in the original input.
    pub input_end: usize,
}

/// Trait for extracting a `SpanSlice` from a pest `Pair`.
pub trait ExtractSpanSlice {
    /// Extracts a `SpanSlice` from the implementing type.
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

/// Errors produced during parsing of Fabulist source code.
#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    /// Expected a specific symbol but found something else.
    #[error("Expected {expected:?}")]
    ExpectedSymbol {
        /// The expected symbol.
        expected: String,
        /// Span covering the unexpected symbol.
        span_slice: SpanSlice,
    },
    /// Invalid number literal.
    #[error("Invalid boolean literal.")]
    InvalidBooleanLiteral(SpanSlice),
    /// Couldn't cast a number literal.
    #[error("Unable to cast number literal.")]
    UnableToCastNumber(SpanSlice),
    /// Invalid primary expression.
    #[error("Invalid primary expression.")]
    InvalidPrimaryExpression(SpanSlice),
    /// Invalid binary operator.
    #[error("Invalid binary operator.")]
    InvalidBinaryOperator(SpanSlice),
    /// Invalid identifier primitive.
    #[error("Invalid identifier primitive.")]
    InvalidIdentifierPrimitive(SpanSlice),
    /// Invalid primitive value.
    #[error("Invalid primitive.")]
    InvalidPrimitive(SpanSlice),
    /// Invalid literal.
    #[error("Invalid literal.")]
    InvalidLiteral(SpanSlice),
    /// Invalid statement.
    #[error("Invalid statement.")]
    InvalidStatement(SpanSlice),
    /// Invalid expression.
    #[error("Invalid expression.")]
    InvalidExpression(SpanSlice),
    /// Invalid definition.
    #[error("Invalid definition.")]
    InvalidDefinition(SpanSlice),
    /// Invalid declaration.
    #[error("Invalid declaration.")]
    InvalidDeclaration(SpanSlice),
    /// Unable to parse story.
    #[error("Unable to parse story.")]
    UnableToParseStory(SpanSlice),
    /// Failed pest parsing.
    #[error("Failed pest parsing: {0}")]
    PestParsing(#[from] Box<pest::error::Error<Rule>>),
}

/// Result type for parser operations.
pub type ParserResult<T> = Result<T, ParserError>;
