use pest::{error::LineColLocation, RuleType, Span};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("[line 0:0] Fabulist can't be parsed")]
    InvalidFabulist,
    #[error("[line {0}:{1}] {2}")]
    ParsingError(usize, usize, String),
}

impl Error {
    pub fn map_parser_error<R>(error: pest::error::Error<R>) -> Error
    where
        R: RuleType,
    {
        let message = error.variant.message();
        let (line, col) = match error.line_col {
            LineColLocation::Pos(line_col) => line_col,
            _ => (0, 0),
        };
        Error::ParsingError(line, col, message.into())
    }

    pub fn map_span(span: Span, message: impl Into<String>) -> Error {
        let line_col = LineColLocation::from(span);
        let (start, _) = match line_col {
            LineColLocation::Span(start, end) => (start, end),
            _ => ((0, 0), (0, 0)),
        };
        let (start_line, start_col) = start;
        Error::ParsingError(start_line, start_col, message.into())
    }
}
