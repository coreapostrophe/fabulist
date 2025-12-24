#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid stack address")]
    InvalidStackAddress,
    #[error("Type mismatch")]
    TypeMismatch,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Constant does not exist")]
    ConstantDoesNotExist,
    #[error("Instruction pointer out of bounds")]
    InstructionOutOfBounds,
}
