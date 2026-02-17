use crate::{error::Error, instructions::Instruction, program::Program, value::Value};

pub mod error;
pub mod instructions;
pub mod program;
pub mod translator;
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
                Instruction::LoadConstant(value) => {
                    self.stack.push(value.clone());
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

                Instruction::Load => {
                    let address = self.stack.last().ok_or(Error::StackUnderflow)?;
                    let value = match address {
                        Value::Address(addr) => self
                            .stack
                            .get(*addr)
                            .ok_or(Error::InvalidStackAddress)?
                            .clone(),
                        _ => return Err(Error::TypeMismatch),
                    };
                    let last_mut = self.stack.last_mut().ok_or(Error::StackUnderflow)?;
                    *last_mut = value;
                }
                Instruction::Store => {
                    let address = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    // TODO: Consider popping as a future optimization
                    let value = self.stack.last().ok_or(Error::StackUnderflow)?.clone();

                    match address {
                        Value::Address(addr) => {
                            let slot =
                                self.stack.get_mut(addr).ok_or(Error::InvalidStackAddress)?;
                            *slot = value;
                        }
                        _ => return Err(Error::TypeMismatch),
                    }
                }

                Instruction::Halt => {
                    break 'runtime;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(42)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(10)),
            Instruction::LoadConstant(Value::Number(32)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(100)),
            Instruction::LoadConstant(Value::Number(58)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(6)),
            Instruction::LoadConstant(Value::Number(7)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(84)),
            Instruction::LoadConstant(Value::Number(2)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(85)),
            Instruction::LoadConstant(Value::Number(43)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Boolean(true)),
            Instruction::LoadConstant(Value::Boolean(false)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Boolean(true)),
            Instruction::LoadConstant(Value::Boolean(false)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(42)),
            Instruction::LoadConstant(Value::Number(42)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(42)),
            Instruction::LoadConstant(Value::Number(43)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(41)),
            Instruction::LoadConstant(Value::Number(42)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(42)),
            Instruction::LoadConstant(Value::Number(42)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(43)),
            Instruction::LoadConstant(Value::Number(42)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(42)),
            Instruction::LoadConstant(Value::Number(42)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(42)),
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
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Boolean(true)),
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

    #[test]
    fn load_instruction_works() {
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(42)), // Push 42 onto the stack
            Instruction::LoadConstant(Value::Address(0)), // Push address 0 onto the stack
            Instruction::Load,                            // Load value at address 0 (first 42)
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 2);

        match &vm.stack[1] {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn store_instruction_works() {
        let program = Program::new(vec![
            Instruction::LoadConstant(Value::Number(0)), // Push  0 onto the stack
            Instruction::LoadConstant(Value::Number(42)), // Push 42 onto the stack
            Instruction::LoadConstant(Value::Address(0)), // Push address 0 onto the stack
            Instruction::Store,                          // Store 42 at address 0
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 2);

        match &vm.stack[0] {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number value"),
        }
    }
}
