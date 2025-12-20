use crate::{bytecode::OpCode, chunk::Chunk, error::Error, value::Value};

pub mod bytecode;
pub mod chunk;
pub mod error;
pub mod value;

macro_rules! binary_op {
    ($self:expr, $operation:tt) => {{
        let right = $self.stack.pop().ok_or(Error::StackUnderflow)?;
        let left = $self.stack.pop().ok_or(Error::StackUnderflow)?;
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                $self.stack.push(Value::Number(l $operation r));
            }
            _ => Err(Error::UnableToCastValue)?,
        }
    }};
}

pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,
    instr_pointer: usize,
    stack: Vec<Value>,
}

impl<'a> VirtualMachine<'a> {
    pub fn interpret(chunk: &'a Chunk) -> Result<Value, Error> {
        let mut vm = Self {
            chunk,
            instr_pointer: 0,
            stack: Vec::with_capacity(256),
        };

        vm.run()
    }

    pub fn run(&mut self) -> Result<Value, Error> {
        'runtime: loop {
            let instruction = self
                .chunk
                .code()
                .get(self.instr_pointer)
                .ok_or(Error::UnexpectedEndOfChunk)?;
            self.instr_pointer += 1;

            match instruction {
                OpCode::Constant(idx) => {
                    let constant = self
                        .chunk
                        .constants()
                        .get(*idx)
                        .ok_or(Error::UnexpectedEndOfChunk)?;
                    self.stack.push(constant.clone());
                }
                OpCode::Negate => {
                    let value = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    match value {
                        Value::Number(n) => {
                            self.stack.push(Value::Number(-n));
                        }
                        _ => Err(Error::UnableToCastValue)?,
                    }
                }
                OpCode::Add => binary_op!(self, +),
                OpCode::Subtract => binary_op!(self, -),
                OpCode::Multiply => binary_op!(self, *),
                OpCode::Divide => binary_op!(self, /),
                OpCode::Return => {
                    break 'runtime Ok(self.stack.pop().unwrap_or(Value::None));
                }
            }
        }
    }
}
