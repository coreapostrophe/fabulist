use crate::{error::Error, instructions::Instruction, program::Program, value::Value};

pub mod error;
pub mod instructions;
pub mod program;
pub mod value;

macro_rules! binary_op {
    ($self:expr, $operation:tt) => {{
        let rhs = $self.stack.pop().ok_or(Error::StackUnderflow)?;
        let lhs_mut = $self.stack.last_mut().ok_or(Error::StackUnderflow)?;

        match (lhs_mut, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                let value = *lhs $operation rhs;
                *lhs = value;
            }
            _ => return Err(Error::TypeMismatch),
        }
    }};
}

macro_rules! binary_logic {
    ($self:expr, $operation:tt) => {{
        let rhs = $self.stack.pop().ok_or(Error::StackUnderflow)?;
        let lhs_mut = $self.stack.last_mut().ok_or(Error::StackUnderflow)?;

        match (lhs_mut, rhs) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => {
                let value = *lhs $operation rhs;
                *lhs = value;
            }
            _ => return Err(Error::TypeMismatch),
        }
    }};
}

macro_rules! binary_bool {
    ($self:expr, $operation:tt) => {{
        let rhs = $self.stack.pop().ok_or(Error::StackUnderflow)?;
        let lhs_mut = $self.stack.last_mut().ok_or(Error::StackUnderflow)?;

        let value = match (lhs_mut.clone(), rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => lhs $operation rhs,
            (Value::Boolean(lhs), Value::Boolean(rhs)) => lhs $operation rhs,
            _ => return Err(Error::TypeMismatch),
        };

        *lhs_mut = Value::Boolean(value);
    }};
}

pub struct VirtualMachine<'a> {
    program: &'a Program,
    stack: Vec<Value>,
    program_counter: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn interpret(program: &'a Program) -> Result<Self, Error> {
        let mut vm = VirtualMachine {
            program,
            stack: Vec::new(),
            program_counter: 0,
        };

        vm.run()?;

        Ok(vm)
    }

    fn run(&mut self) -> Result<(), Error> {
        'runtime: loop {
            let instruction = self
                .program
                .get_instruction(self.program_counter)
                .ok_or(Error::InstructionOutOfBounds)?;

            self.program_counter += 1;

            match instruction {
                Instruction::LoadConstant(index) => {
                    let constant = self
                        .program
                        .get_constant(*index)
                        .ok_or(Error::ConstantDoesNotExist)?;
                    self.stack.push(constant.clone());
                }

                Instruction::Add => binary_op!(self, +),
                Instruction::Sub => binary_op!(self, -),
                Instruction::Mul => binary_op!(self, *),
                Instruction::Div => binary_op!(self, /),
                Instruction::Mod => binary_op!(self, %),

                Instruction::Neg => {
                    let value_mut = self.stack.last_mut().ok_or(Error::StackUnderflow)?;

                    match value_mut {
                        Value::Number(n) => {
                            *n = -*n;
                        }
                        _ => return Err(Error::TypeMismatch),
                    }
                }
                Instruction::Not => {
                    let value_mut = self.stack.last_mut().ok_or(Error::StackUnderflow)?;

                    match value_mut {
                        Value::Boolean(b) => {
                            *b = !*b;
                        }
                        _ => return Err(Error::TypeMismatch),
                    }
                }

                Instruction::And => binary_logic!(self, &&),
                Instruction::Or => binary_logic!(self, ||),

                Instruction::Eq => binary_bool!(self, ==),
                Instruction::Neq => binary_bool!(self, !=),
                Instruction::Le => binary_bool!(self, <),
                Instruction::Leq => binary_bool!(self, <=),
                Instruction::Gr => binary_bool!(self, >),
                Instruction::Geq => binary_bool!(self, >=),

                Instruction::Halt => {
                    break 'runtime;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod vm_tests {
    use super::*;
    use crate::{instructions::Instruction, value::Value};

    #[test]
    fn halt_instruction_works() {
        let program = Program::new(vec![Instruction::Halt]);

        let vm = VirtualMachine::interpret(&program);
        assert!(vm.is_ok())
    }

    #[test]
    fn load_constant_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index = program.add_constant(Value::Number(42));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index),
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn add_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(10));
        let const_index2 = program.add_constant(Value::Number(32));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Add,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn sub_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(100));
        let const_index2 = program.add_constant(Value::Number(58));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Sub,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn mul_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(6));
        let const_index2 = program.add_constant(Value::Number(7));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Mul,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn div_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(84));
        let const_index2 = program.add_constant(Value::Number(2));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Div,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn mod_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(85));
        let const_index2 = program.add_constant(Value::Number(43));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Mod,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn and_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Boolean(true));
        let const_index2 = program.add_constant(Value::Boolean(false));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::And,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Boolean(b) => assert!(!*b),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn or_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Boolean(true));
        let const_index2 = program.add_constant(Value::Boolean(false));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Or,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Boolean(b) => assert!(*b),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn eq_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(42));
        let const_index2 = program.add_constant(Value::Number(42));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Eq,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Boolean(b) => assert!(*b),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn neq_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(42));
        let const_index2 = program.add_constant(Value::Number(43));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Neq,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Boolean(b) => assert!(*b),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn le_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(41));
        let const_index2 = program.add_constant(Value::Number(42));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Le,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Boolean(b) => assert!(*b),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn leq_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(42));
        let const_index2 = program.add_constant(Value::Number(42));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Leq,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Boolean(b) => assert!(*b),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn gr_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(43));
        let const_index2 = program.add_constant(Value::Number(42));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Gr,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Boolean(b) => assert!(*b),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn geq_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index1 = program.add_constant(Value::Number(42));
        let const_index2 = program.add_constant(Value::Number(42));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index1),
            Instruction::LoadConstant(const_index2),
            Instruction::Geq,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Boolean(b) => assert!(*b),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn neg_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index = program.add_constant(Value::Number(42));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index),
            Instruction::Neg,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, -42),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn not_instruction_works() {
        let mut program = Program::new(vec![]);
        let const_index = program.add_constant(Value::Boolean(true));

        program.write_instructions(vec![
            Instruction::LoadConstant(const_index),
            Instruction::Not,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 1);

        match &vm.stack[0] {
            Value::Boolean(b) => assert!(!*b),
            _ => panic!("Expected Boolean value"),
        }
    }
}
