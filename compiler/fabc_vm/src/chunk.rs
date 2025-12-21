use crate::{bytecode::OpCode, value::Value};

#[derive(Default)]
pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn code(&self) -> &Vec<OpCode> {
        &self.code
    }

    pub fn constants(&self) -> &Vec<Value> {
        &self.constants
    }

    pub fn write_opcode(&mut self, opcode: OpCode) {
        self.code.push(opcode);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
