#![allow(unused)]
use std::collections::HashMap;

use fabc_error::Error;
use fabc_parser::Parsable;

use crate::{instructions::Instruction, procedures::Procedure};

pub mod implementations;
pub mod instructions;
pub mod procedures;
pub mod tac;

pub trait GenerateIR {
    fn generate_ir(&self, generator: &mut IRGenerator);
}

#[derive(Default)]
pub struct IRGenerator {
    procedures: Vec<Procedure>,
    labels: HashMap<String, usize>,
    instructions: Vec<Instruction>,
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
    pub(crate) fn add_procedure(&mut self, procedure: Procedure) -> &mut Self {
        self.procedures.push(procedure);
        self
    }
    pub(crate) fn add_label(&mut self, name: String, index: usize) -> &mut Self {
        self.labels.insert(name, index);
        self
    }
    pub(crate) fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }
    pub(crate) fn add_instructions(&mut self, instructions: Vec<Instruction>) -> &mut Self {
        self.instructions.extend(instructions);
        self
    }
}
