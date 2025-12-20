pub enum OpCode {
    Constant(usize),

    // Arithmetic
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,

    Return,
}
