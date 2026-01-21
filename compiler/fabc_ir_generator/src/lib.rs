#![allow(unused)]
use fabc_parser::Parsable;

pub mod implementations;
pub mod ssa;

pub use ssa::cfg::Procedure;
pub use ssa::instr::{Instruction, Terminator};
pub use ssa::{BlockId, BlockParam, Literal, Operand, PhiNode, ValueId};

pub trait GenerateIR {
    fn generate_ir(&self, generator: &mut IRGenerator);
}

#[derive(Default)]
pub struct IRGenerator {
    procedures: Vec<Procedure>,
    next_value: usize,
    next_block: usize,
}

impl IRGenerator {
    pub fn new() -> Self {
        IRGenerator::default()
    }

    pub fn generate<T>(ast: T) -> Self
    where
        T: Parsable + GenerateIR,
    {
        let mut generator = IRGenerator::new();
        ast.generate_ir(&mut generator);
        generator
    }

    pub fn procedures(&self) -> &[Procedure] {
        &self.procedures
    }

    pub fn into_procedures(self) -> Vec<Procedure> {
        self.procedures
    }

    pub(crate) fn fresh_value(&mut self) -> ValueId {
        let id = ValueId(self.next_value);
        self.next_value += 1;
        id
    }

    pub(crate) fn fresh_block(&mut self) -> BlockId {
        let id = BlockId(self.next_block);
        self.next_block += 1;
        id
    }

    pub(crate) fn make_param(&mut self, hint: Option<String>) -> BlockParam {
        let id = self.fresh_value();
        match hint {
            Some(name) => BlockParam::with_hint(id, name),
            None => BlockParam::new(id),
        }
    }

    pub(crate) fn add_procedure(&mut self, procedure: Procedure) -> &mut Self {
        self.procedures.push(procedure);
        self
    }
}
