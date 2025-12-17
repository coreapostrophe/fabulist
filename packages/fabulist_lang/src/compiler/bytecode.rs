#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(usize),
    LoadNumber(f32),
    LoadBool(bool),
    LoadNone,

    GetLocal(usize),
    SetLocal(usize),
    GetGlobal(String),
    SetGlobal(String),

    GetContext(String),
    SetContext(String),

    Add,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,

    Negate,
    Not,

    Jump(usize),
    JumpIfFalse(usize),
    Goto(String),

    Call(usize),
    MakeLambda(usize),
    Return,
}
