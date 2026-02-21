use crate::value::Value;

#[derive(Debug)]
pub enum Instruction {
    LoadConstant(Value),
    Load,
    Store,

    Add,
    Mul,
    Div,
    Sub,
    Mod,

    Neg,
    Not,

    And,
    Or,

    Eq,
    Neq,
    Le,
    Leq,
    Gr,
    Geq,

    Halt,
}
