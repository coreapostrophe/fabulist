use super::{BlockId, Operand, ValueId};
use fabc_parser::ast::expr::{BinaryOperator, UnaryOperator};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Binary {
        op: BinaryOperator,
        lhs: Operand,
        rhs: Operand,
        dest: ValueId,
    },
    Unary {
        op: UnaryOperator,
        arg: Operand,
        dest: ValueId,
    },
    Copy {
        src: Operand,
        dest: ValueId,
    },
    Call {
        procedure: String,
        args: Vec<Operand>,
        dest: Option<ValueId>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Terminator {
    Jump {
        target: BlockId,
        args: Vec<Operand>,
    },
    Branch {
        condition: Operand,
        then_target: BlockId,
        else_target: BlockId,
        then_args: Vec<Operand>,
        else_args: Vec<Operand>,
    },
    Return {
        value: Option<Operand>,
    },
    Unreachable,
}
