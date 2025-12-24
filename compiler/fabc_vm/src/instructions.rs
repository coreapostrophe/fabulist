use crate::value::Value;

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
