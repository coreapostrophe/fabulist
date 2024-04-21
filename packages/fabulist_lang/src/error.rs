use pest::{error::LineColLocation, RuleType, Span};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("[Error {line}:{col}] {message}")]
    ParsingError {
        line: usize,
        col: usize,
        line_col: LineColLocation,
        message: String,
    },
}

impl Error {
    pub fn map_parser_error<R>(error: pest::error::Error<R>) -> Error
    where
        R: RuleType,
    {
        let message = error.variant.message();
        let line_col = error.line_col;
        let (line, col) = match line_col {
            LineColLocation::Pos(line_col) => line_col,
            _ => (0, 0),
        };
        Error::ParsingError {
            line,
            col,
            line_col,
            message: message.into(),
        }
    }

    pub fn map_span(span: Span, message: impl Into<String>) -> Error {
        let line_col = LineColLocation::from(span);
        let (start, _) = match line_col {
            LineColLocation::Span(start, end) => (start, end),
            _ => ((0, 0), (0, 0)),
        };
        let (line, col) = start;
        Error::ParsingError {
            line,
            col,
            line_col,
            message: message.into(),
        }
    }
}
