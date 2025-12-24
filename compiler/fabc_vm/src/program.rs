use crate::{instructions::Instruction, value::Value};

pub struct Program {
    store: Vec<Instruction>,
    constants: Vec<Value>,
}

impl Program {
    pub fn new(store: Vec<Instruction>) -> Self {
        Program {
            store,
            constants: Vec::new(),
        }
    }

    pub fn write_instructions(&mut self, instructions: Vec<Instruction>) {
        self.store.extend(instructions);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }

    pub fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.store.get(index)
    }
}
