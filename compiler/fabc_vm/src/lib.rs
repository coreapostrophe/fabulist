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
    pub fn interpret(chunk: &'a Chunk) -> Result<Self, Error> {
        let mut vm = Self {
            chunk,
            instr_pointer: 0,
            stack: Vec::with_capacity(256),
        };

        vm.run()?;

        Ok(vm)
    }

    pub fn run(&mut self) -> Result<(), Error> {
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
                    break 'runtime;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod vm_tests {
    use crate::{bytecode::OpCode, chunk::Chunk, value::Value, VirtualMachine};

    #[test]
    fn return_operation_works() {
        let mut chunk = Chunk::new();
        chunk.write_opcode(OpCode::Return);

        let result = VirtualMachine::interpret(&chunk);
        assert!(result.is_ok());
    }

    #[test]
    fn constant_operation_works() {
        let mut chunk = Chunk::new();
        let const_idx = chunk.add_constant(Value::Number(42.0));

        chunk.write_opcode(OpCode::Constant(const_idx));
        chunk.write_opcode(OpCode::Return);

        let result = VirtualMachine::interpret(&chunk);
        assert!(result.is_ok());

        let vm = result.expect("Failed to interpret chunk");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 42.0),
            _ => panic!("Expected a number on the stack"),
        }
    }

    #[test]
    fn binary_add_operation_works() {
        let mut chunk = Chunk::new();
        let const_idx1 = chunk.add_constant(Value::Number(10.0));
        let const_idx2 = chunk.add_constant(Value::Number(5.0));

        chunk.write_opcode(OpCode::Constant(const_idx1));
        chunk.write_opcode(OpCode::Constant(const_idx2));
        chunk.write_opcode(OpCode::Add);
        chunk.write_opcode(OpCode::Return);

        let result = VirtualMachine::interpret(&chunk);
        assert!(result.is_ok());

        let vm = result.expect("Failed to interpret chunk");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 15.0),
            _ => panic!("Expected a number on the stack"),
        }
    }

    #[test]
    fn binary_subtract_operation_works() {
        let mut chunk = Chunk::new();
        let const_idx1 = chunk.add_constant(Value::Number(10.0));
        let const_idx2 = chunk.add_constant(Value::Number(5.0));

        chunk.write_opcode(OpCode::Constant(const_idx1));
        chunk.write_opcode(OpCode::Constant(const_idx2));
        chunk.write_opcode(OpCode::Subtract);
        chunk.write_opcode(OpCode::Return);

        let result = VirtualMachine::interpret(&chunk);
        assert!(result.is_ok());

        let vm = result.expect("Failed to interpret chunk");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 5.0),
            _ => panic!("Expected a number on the stack"),
        }
    }

    #[test]
    fn binary_multiply_operation_works() {
        let mut chunk = Chunk::new();
        let const_idx1 = chunk.add_constant(Value::Number(10.0));
        let const_idx2 = chunk.add_constant(Value::Number(5.0));

        chunk.write_opcode(OpCode::Constant(const_idx1));
        chunk.write_opcode(OpCode::Constant(const_idx2));
        chunk.write_opcode(OpCode::Multiply);
        chunk.write_opcode(OpCode::Return);

        let result = VirtualMachine::interpret(&chunk);
        assert!(result.is_ok());

        let vm = result.expect("Failed to interpret chunk");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 50.0),
            _ => panic!("Expected a number on the stack"),
        }
    }

    #[test]
    fn binary_divide_operation_works() {
        let mut chunk = Chunk::new();
        let const_idx1 = chunk.add_constant(Value::Number(10.0));
        let const_idx2 = chunk.add_constant(Value::Number(5.0));

        chunk.write_opcode(OpCode::Constant(const_idx1));
        chunk.write_opcode(OpCode::Constant(const_idx2));
        chunk.write_opcode(OpCode::Divide);
        chunk.write_opcode(OpCode::Return);

        let result = VirtualMachine::interpret(&chunk);
        assert!(result.is_ok());

        let vm = result.expect("Failed to interpret chunk");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 2.0),
            _ => panic!("Expected a number on the stack"),
        }
    }

    #[test]
    fn negate_operation_works() {
        let mut chunk = Chunk::new();
        let const_idx = chunk.add_constant(Value::Number(42.0));

        chunk.write_opcode(OpCode::Constant(const_idx));
        chunk.write_opcode(OpCode::Negate);
        chunk.write_opcode(OpCode::Return);

        let result = VirtualMachine::interpret(&chunk);
        assert!(result.is_ok());

        let vm = result.expect("Failed to interpret chunk");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, -42.0),
            _ => panic!("Expected a number on the stack"),
        }
    }
}
