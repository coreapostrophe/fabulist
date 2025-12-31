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
    #[error("Unexpected element")]
    UnhandledElement,
    #[error("Unexpected literal")]
    UnhandledLiteral,
    #[error("Unexpected primary expression")]
    UnhandledPrimaryExpression,
    #[error("Unhandled initiator")]
    UnhandledInitiator,
    #[error("Invalid logical operator")]
    InvalidLogicalOperator,
    #[error("Invalid unary operator")]
    InvalidUnaryOperator,
    #[error("Invalid binary operator")]
    InvalidBinaryOperator,
    #[error("Expected `{expected}`, found `{found}`")]
    ExpectedFound { expected: String, found: String },
    #[error("Lexer error: {0}")]
    LexerError(#[from] fabc_lexer::error::Error),
}
