use crate::{compiler::bytecode::OpCode, interpreter::runtime_value::RuntimeValue};

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<RuntimeValue>,
    pub lines: Vec<usize>,
}
