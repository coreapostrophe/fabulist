#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unexpected keyword literal")]
    UnhandledKeywordLiteral,
    #[error("Unexpected primary expression")]
    UnhandledPrimaryExpression,
    #[error("Expected `{0}` in expression")]
    UnclosedDelimiter(String),
}
