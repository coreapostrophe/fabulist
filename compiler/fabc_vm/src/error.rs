#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to cast value")]
    UnableToCastValue,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Unexpected end of chunk")]
    UnexpectedEndOfChunk,
}
