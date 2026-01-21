pub mod cfg;
pub mod instr;

pub use cfg::{BasicBlock, Procedure};
pub use instr::{Instruction, Terminator};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(i64),
    Boolean(bool),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Value(ValueId),
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockParam {
    pub id: ValueId,
    pub hint: Option<String>,
}

impl BlockParam {
    pub fn new(id: ValueId) -> Self {
        BlockParam { id, hint: None }
    }

    pub fn with_hint(id: ValueId, hint: impl Into<String>) -> Self {
        BlockParam {
            id,
            hint: Some(hint.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhiNode {
    pub destination: ValueId,
    pub sources: Vec<(BlockId, Operand)>,
}

impl PhiNode {
    pub fn new(destination: ValueId) -> Self {
        PhiNode {
            destination,
            sources: Vec::new(),
        }
    }

    pub fn with_sources(destination: ValueId, sources: Vec<(BlockId, Operand)>) -> Self {
        PhiNode {
            destination,
            sources,
        }
    }

    pub fn add_source(&mut self, block: BlockId, operand: Operand) -> &mut Self {
        self.sources.push((block, operand));
        self
    }
}
