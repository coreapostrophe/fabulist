use pest::{error::LineColLocation, Span};

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
    pub fn map_span(span: Span, message: impl Into<String>) -> Error {
        let line_col = LineColLocation::from(span);
        Self::map_line_col(line_col, message)
    }
    pub fn map_line_col(line_col: LineColLocation, message: impl Into<String>) -> Error {
        match line_col {
            LineColLocation::Span((line, col), _) => Error::ParsingError {
                line,
                col,
                line_col,
                message: message.into(),
            },
            LineColLocation::Pos((line, col)) => Error::ParsingError {
                line,
                col,
                line_col,
                message: message.into(),
            },
        }
    }
}
