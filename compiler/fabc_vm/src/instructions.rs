use crate::value::Value;

#[derive(Debug)]
pub enum Instruction {
    EnterFrame(usize),
    ExitFrame,

    LoadConstant(Value),
    LoadLocal(usize),
    StoreLocal(usize),

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
