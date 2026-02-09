#![allow(unused)]
use std::collections::HashMap;

use fabc_error::Error;

use fabc_parser::Parsable;

pub mod implementations;
pub mod quadruple;

pub use quadruple::{
    Block, LabelId, Literal, Operand, Param, Procedure, Quadruple, TempId, Terminator,
};

pub trait GenerateIR {
    fn generate_ir(&self, generator: &mut IRGenerator) -> IRResult;
}

#[derive(Default)]
pub struct IRGenerator {
    procedures: Vec<Procedure>,
    next_temp: usize,
    next_label: usize,
    symbols: HashMap<String, TempId>,
    errors: Vec<Error>,
}

#[derive(Default)]
pub struct IRResult {
    pub operand: Option<Operand>,
    pub quadruples: Vec<Quadruple>,
}

impl IRResult {
    pub fn with_operand(operand: Operand) -> Self {
        Self {
            operand: Some(operand),
            quadruples: Vec::new(),
        }
    }

    pub fn merge(mut self, mut other: IRResult) -> Self {
        self.quadruples.append(&mut other.quadruples);
        if self.operand.is_none() {
            self.operand = other.operand;
        }
        self
    }
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
        let _ = ast.generate_ir(&mut generator);
        generator
    }

    pub fn procedures(&self) -> &[Procedure] {
        &self.procedures
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    pub fn into_errors(self) -> Vec<Error> {
        self.errors
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

    pub(crate) fn temp_for_symbol(&mut self, name: impl Into<String>) -> TempId {
        let name = name.into();
        if let Some(existing) = self.symbols.get(&name) {
            return *existing;
        }

        let temp = self.fresh_temp();
        self.symbols.insert(name, temp);
        temp
    }

    pub(crate) fn add_procedure(&mut self, procedure: Procedure) -> &mut Self {
        self.procedures.push(procedure);
        self
    }

    pub(crate) fn push_error(&mut self, error: Error) {
        self.errors.push(error);
    }
}
