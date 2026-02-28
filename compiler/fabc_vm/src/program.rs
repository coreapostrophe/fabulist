use crate::instructions::Instruction;

pub struct Program {
    store: Vec<Instruction>,
    global_count: usize,
}

impl Program {
    pub fn new(store: Vec<Instruction>) -> Self {
        let global_count = store
            .iter()
            .filter_map(|instr| match instr {
                Instruction::LoadGlobal(slot) | Instruction::StoreGlobal(slot) => Some(*slot),
                _ => None,
            })
            .max()
            .map(|max_slot| max_slot + 1)
            .unwrap_or(0);

        Program {
            store,
            global_count,
        }
    }

    pub fn write_instructions(&mut self, instructions: Vec<Instruction>) {
        self.store.extend(instructions);
        let extra_max = self
            .store
            .iter()
            .filter_map(|instr| match instr {
                Instruction::LoadGlobal(slot) | Instruction::StoreGlobal(slot) => Some(*slot),
                _ => None,
            })
            .max();
        if let Some(max_slot) = extra_max {
            self.global_count = self.global_count.max(max_slot + 1);
        }
    }

    pub fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.store.get(index)
    }

    pub fn global_count(&self) -> usize {
        self.global_count
    }
}
