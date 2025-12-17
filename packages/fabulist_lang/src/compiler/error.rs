#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error("Expected a {0} type.")]
    TypeError(String),
    #[error("VM Stack is empty when attempting to pop a value.")]
    StackIsEmpty,
}
