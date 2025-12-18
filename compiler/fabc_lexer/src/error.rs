#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unterminated string literal")]
    UnterminatedString,
    #[error("Unable to parse number literal")]
    UnableToParseNumber,
}
