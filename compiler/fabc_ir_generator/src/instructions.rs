use crate::tac::Address;
use fabc_parser::ast::expr::{BinaryOperator, UnaryOperator};

pub struct Label(String);

impl Label {
    pub fn new(name: impl Into<String>) -> Self {
        Label(name.into())
    }
    pub fn id(&self) -> &str {
        &self.0
    }
}

pub enum Instruction {
    Binary {
        op: BinaryOperator,
        result: Address,
        arg1: Address,
        arg2: Address,
    },
    Unary {
        op: UnaryOperator,
        result: Address,
        arg: Address,
    },
    Copy {
        result: Address,
        arg: Address,
    },
    Jump {
        target: Label,
    },
    ConditionalJump {
        condition: Address,
        target: Label,
        else_target: Label,
    },
    Parameter {
        arg: Address,
    },
    Call {
        procedure: String,
        arity: usize,
    },
}
