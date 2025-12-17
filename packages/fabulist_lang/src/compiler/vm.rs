use std::collections::HashMap;

use crate::{
    compiler::{bytecode::OpCode, chunk::Chunk, error::CompilerError, value::Value},
    interpreter::environment::RuntimeEnvironment,
};

macro_rules! binary_op {
    ($self:expr, $($op:tt)+) => {
        {
            let right = $self.stack.pop().ok_or(CompilerError::StackIsEmpty)?;
            let left = $self.stack.pop().ok_or(CompilerError::StackIsEmpty)?;

            if let (Value::Number(left_num), Value::Number(right_num)) = (left, right) {
                $self.stack.push(Value::Number(left_num $($op)+ right_num));
            } else {
                return Err(CompilerError::TypeError("Number".to_string()));
            }
        }
    };
}

pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,
    instr_pointer: usize,
    stack: Vec<Value>,
    _globals: HashMap<String, Value>,
    _context: RuntimeEnvironment,
}

impl<'a> VirtualMachine<'a> {
    pub fn interpret(chunk: &'a Chunk) -> Result<Value, CompilerError> {
        let mut vm = VirtualMachine {
            chunk,
            instr_pointer: 0,
            stack: Vec::new(),
            _globals: HashMap::new(),
            _context: RuntimeEnvironment::new(),
        };

        vm.run()
    }

    pub fn run(&mut self) -> Result<Value, CompilerError> {
        'beat: loop {
            let instruction = &self.chunk.code[self.instr_pointer];
            self.instr_pointer += 1;

            match instruction {
                OpCode::Subtraction => binary_op!(self, -),
                OpCode::Division => binary_op!(self, /),
                OpCode::Multiplication => binary_op!(self, *),
                OpCode::Add => binary_op!(self, +),
                OpCode::Negate => {
                    let value = self.stack.pop().ok_or(CompilerError::StackIsEmpty)?;
                    if let Value::Number(num) = value {
                        self.stack.push(Value::Number(-num));
                    } else {
                        return Err(CompilerError::TypeError("Number".to_string()));
                    }
                }
                OpCode::Return => {
                    break 'beat;
                }
                _ => todo!("Implement other opcodes"),
            }
        }

        todo!()
    }
}
