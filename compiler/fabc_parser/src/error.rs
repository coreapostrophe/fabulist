#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unexpected keyword literal")]
    UnhandledKeywordLiteral,
    #[error("Unexpected primitive")]
    UnhandledPrimitive,
    #[error("Unexpected primary")]
    UnhandledPrimary,
    #[error("Unexpected literal")]
    UnhandledLiteral,
    #[error("Unexpected primary expression")]
    UnhandledPrimaryExpression,
    #[error("Invalid unary operator")]
    InvalidUnaryOperator,
    #[error("Invalid binary operator")]
    InvalidBinaryOperator,
    #[error("Expected `{0}`, found `{1}`")]
    ExpectedFound(String, String),
}
