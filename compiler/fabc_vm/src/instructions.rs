pub enum Instruction {
    LoadConstant(usize),

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
