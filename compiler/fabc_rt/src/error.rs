use std::result::Result as StdResult;

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    #[error("story has reached the end")]
    EndOfStory,
    #[error("current step requires a choice index")]
    ChoiceExpected,
    #[error("current step is not a selection")]
    NotInSelection,
    #[error("choice index {index} is out of bounds for {len} choices")]
    InvalidChoice { index: usize, len: usize },
    #[error("story part `{0}` does not exist")]
    UnknownPart(String),
    #[error("undefined variable `{0}`")]
    UndefinedVariable(String),
    #[error("invalid callable value `{0}`")]
    InvalidCallee(String),
    #[error("closure expected {expected} arguments but received {got}")]
    ArityMismatch { expected: usize, got: usize },
    #[error("invalid assignment target")]
    InvalidAssignmentTarget,
    #[error("cannot read member `{member}` from `{target}`")]
    InvalidMemberAccess { target: String, member: String },
    #[error("invalid member key from `{0}`")]
    InvalidMemberKey(String),
    #[error("cannot use `{0}` as a story target")]
    InvalidStoryTarget(String),
    #[error("cannot cast `{0}` to a number")]
    InvalidNumberCast(String),
    #[error("cannot cast `{0}` to a boolean")]
    InvalidBooleanCast(String),
    #[error("invalid operation `{operator}` for `{left}` and `{right}`")]
    InvalidBinaryOperation {
        operator: &'static str,
        left: String,
        right: String,
    },
    #[error("native closure execution failed: {0}")]
    NativeExecution(String),
    #[error("unexpected control flow while evaluating metadata")]
    UnexpectedControlFlow,
}

pub type Result<T> = StdResult<T, RuntimeError>;
