use crate::{instructions::Instruction, tac::Address};

#[derive(Default)]
pub struct Procedure {
    params: Vec<Address>,
    body: Vec<Instruction>,
}

impl Procedure {
    pub fn new(params: Vec<Address>, body: Vec<Instruction>) -> Self {
        Procedure { params, body }
    }
    pub fn add_param(&mut self, param: Address) -> &mut Self {
        self.params.push(param);
        self
    }
    pub fn add_params(&mut self, params: Vec<Address>) -> &mut Self {
        self.params.extend(params);
        self
    }
    pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.body.push(instruction);
        self
    }
    pub fn add_instructions(&mut self, instructions: Vec<Instruction>) -> &mut Self {
        self.body.extend(instructions);
        self
    }
}
