#![allow(unused)]
use fabc_parser::Parsable;

pub mod implementations;
pub mod quadruple;

pub use quadruple::{
    Block, LabelId, Literal, Operand, Param, Procedure, Quadruple, TempId, Terminator,
};

pub trait GenerateIR {
    fn generate_ir(&self, generator: &mut IRGenerator);
}

#[derive(Default)]
pub struct IRGenerator {
    procedures: Vec<Procedure>,
    next_temp: usize,
    next_label: usize,
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

    pub(crate) fn fresh_temp(&mut self) -> TempId {
        let id = TempId(self.next_temp);
        self.next_temp += 1;
        id
    }

    pub(crate) fn fresh_label(&mut self) -> LabelId {
        let id = LabelId(self.next_label);
        self.next_label += 1;
        id
    }

    pub(crate) fn make_param(&mut self, hint: Option<String>) -> Param {
        let id = self.fresh_temp();
        match hint {
            Some(name) => Param::with_hint(id, name),
            None => Param::new(id),
        }
    }

    pub(crate) fn add_procedure(&mut self, procedure: Procedure) -> &mut Self {
        self.procedures.push(procedure);
        self
    }
}
