use crate::instructions::Instruction;

pub struct Program {
    store: Vec<Instruction>,
}

impl Program {
    pub fn new(store: Vec<Instruction>) -> Self {
        Program { store }
    }

    pub fn write_instructions(&mut self, instructions: Vec<Instruction>) {
        self.store.extend(instructions);
    }

    pub fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.store.get(index)
    }
}
