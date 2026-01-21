use super::{
    instr::{Instruction, Terminator},
    BlockId, BlockParam, PhiNode,
};

#[derive(Debug, PartialEq)]
pub struct BasicBlock {
    pub id: BlockId,
    pub label: Option<String>,
    pub params: Vec<BlockParam>,
    pub phis: Vec<PhiNode>,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Terminator>,
}

impl BasicBlock {
    pub fn new(id: BlockId) -> Self {
        BasicBlock {
            id,
            label: None,
            params: Vec::new(),
            phis: Vec::new(),
            instructions: Vec::new(),
            terminator: None,
        }
    }

    pub fn with_label(id: BlockId, label: impl Into<String>) -> Self {
        BasicBlock {
            label: Some(label.into()),
            ..BasicBlock::new(id)
        }
    }

    pub fn add_param(&mut self, param: BlockParam) -> &mut Self {
        self.params.push(param);
        self
    }

    pub fn add_phi(&mut self, phi: PhiNode) -> &mut Self {
        self.phis.push(phi);
        self
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }

    pub fn add_instructions(&mut self, instructions: Vec<Instruction>) -> &mut Self {
        self.instructions.extend(instructions);
        self
    }

    pub fn set_terminator(&mut self, terminator: Terminator) -> &mut Self {
        self.terminator = Some(terminator);
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct Procedure {
    pub name: Option<String>,
    pub params: Vec<BlockParam>,
    pub blocks: Vec<BasicBlock>,
    pub entry: Option<BlockId>,
}

impl Procedure {
    pub fn new(name: impl Into<Option<String>>) -> Self {
        Procedure {
            name: name.into(),
            params: Vec::new(),
            blocks: Vec::new(),
            entry: None,
        }
    }

    pub fn add_param(&mut self, param: BlockParam) -> &mut Self {
        self.params.push(param);
        self
    }

    pub fn add_params(&mut self, params: Vec<BlockParam>) -> &mut Self {
        self.params.extend(params);
        self
    }

    pub fn add_block(&mut self, block: BasicBlock) -> &mut Self {
        self.blocks.push(block);
        self
    }

    pub fn add_blocks(&mut self, blocks: Vec<BasicBlock>) -> &mut Self {
        self.blocks.extend(blocks);
        self
    }

    pub fn set_entry(&mut self, entry: BlockId) -> &mut Self {
        self.entry = Some(entry);
        self
    }

    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut BasicBlock> {
        self.blocks.iter_mut().find(|block| block.id == id)
    }
}
