use crate::{instructions::Instruction, program::Program, value::Value};
use fabc_error::kind::RuntimeErrorKind as Error;

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

struct Frame {
    locals: Vec<Value>,
}

pub struct VirtualMachine<'a> {
    program: &'a Program,
    stack: Vec<Value>,
    frames: Vec<Frame>,
    program_counter: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn interpret(program: &'a Program) -> Result<Self, Error> {
        let mut vm = VirtualMachine {
            program,
            stack: Vec::new(),
            frames: Vec::new(),
            program_counter: 0,
        };

        vm.run()?;

        Ok(vm)
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn last_value(&self) -> Option<&Value> {
        self.stack.last()
    }

    fn run(&mut self) -> Result<(), Error> {
        'runtime: loop {
            let instruction = self
                .program
                .get_instruction(self.program_counter)
                .ok_or(Error::InstructionOutOfBounds)?;

            self.program_counter += 1;

            match instruction {
                Instruction::EnterFrame(local_count) => {
                    let mut locals = Vec::with_capacity(*local_count);
                    locals.resize(*local_count, Value::None);
                    self.frames.push(Frame { locals });
                }

                Instruction::ExitFrame => {
                    self.frames.pop().ok_or(Error::StackUnderflow)?;
                }

                Instruction::LoadConstant(value) => self.stack.push(value.clone()),

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

                Instruction::LoadLocal(index) => {
                    let frame = self.frames.last().ok_or(Error::StackUnderflow)?;
                    let value = frame
                        .locals
                        .get(*index)
                        .ok_or(Error::InvalidLocalAddress)?
                        .clone();
                    self.stack.push(value);
                }
                Instruction::StoreLocal(index) => {
                    let value = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let frame = self.frames.last_mut().ok_or(Error::StackUnderflow)?;
                    let slot = frame
                        .locals
                        .get_mut(*index)
                        .ok_or(Error::InvalidLocalAddress)?;
                    *slot = value;
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
    fn frame_and_locals_work() {
        let program = Program::new(vec![
            Instruction::EnterFrame(1),
            Instruction::LoadLocal(0),
            Instruction::LoadConstant(Value::Number(42)),
            Instruction::StoreLocal(0),
            Instruction::LoadLocal(0),
            Instruction::ExitFrame,
            Instruction::Halt,
        ]);

        let vm = VirtualMachine::interpret(&program).expect("Failed to interpret program");
        assert_eq!(vm.stack.len(), 2);

        match &vm.stack[0] {
            Value::None => {}
            _ => panic!("Expected initial None local value"),
        }

        match &vm.stack[1] {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number value"),
        }
    }
}
