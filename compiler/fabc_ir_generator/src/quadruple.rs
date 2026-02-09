use fabc_parser::ast::expr::{BinaryOperator, UnaryOperator};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TempId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LabelId(pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Temp(TempId),
    Literal(Literal),
    Context,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub id: TempId,
    pub hint: Option<String>,
}

impl Param {
    pub fn new(id: TempId) -> Self {
        Param { id, hint: None }
    }

    pub fn with_hint(id: TempId, hint: impl Into<String>) -> Self {
        Param {
            id,
            hint: Some(hint.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Quadruple {
    Binary {
        op: BinaryOperator,
        lhs: Operand,
        rhs: Operand,
        dest: TempId,
    },
    Unary {
        op: UnaryOperator,
        arg: Operand,
        dest: TempId,
    },
    Copy {
        src: Operand,
        dest: TempId,
    },
    Call {
        callee: Operand,
        args: Vec<Operand>,
        dest: Option<TempId>,
    },
    MemberAccess {
        base: Operand,
        member: String,
        dest: TempId,
    },
    BuildObject {
        fields: Vec<(String, Operand)>,
        dest: TempId,
    },
    MakeClosure {
        target: LabelId,
        params: Vec<Param>,
        dest: TempId,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Terminator {
    Jump {
        target: LabelId,
    },
    Branch {
        condition: Operand,
        then_target: LabelId,
        else_target: LabelId,
    },
    Return {
        value: Option<Operand>,
    },
    Unreachable,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub label: LabelId,
    pub name: Option<String>,
    pub quadruples: Vec<Quadruple>,
    pub terminator: Option<Terminator>,
}

impl Block {
    pub fn new(label: LabelId) -> Self {
        Block {
            label,
            name: None,
            quadruples: Vec::new(),
            terminator: None,
        }
    }

    pub fn with_name(label: LabelId, name: impl Into<String>) -> Self {
        Block {
            name: Some(name.into()),
            ..Block::new(label)
        }
    }

    pub fn add_quadruple(&mut self, quad: Quadruple) -> &mut Self {
        self.quadruples.push(quad);
        self
    }

    pub fn add_quadruples(&mut self, quads: Vec<Quadruple>) -> &mut Self {
        self.quadruples.extend(quads);
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
    pub params: Vec<Param>,
    pub blocks: Vec<Block>,
    pub entry: Option<LabelId>,
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

    pub fn add_param(&mut self, param: Param) -> &mut Self {
        self.params.push(param);
        self
    }

    pub fn add_params(&mut self, params: Vec<Param>) -> &mut Self {
        self.params.extend(params);
        self
    }

    pub fn add_block(&mut self, block: Block) -> &mut Self {
        self.blocks.push(block);
        self
    }

    pub fn add_blocks(&mut self, blocks: Vec<Block>) -> &mut Self {
        self.blocks.extend(blocks);
        self
    }

    pub fn set_entry(&mut self, entry: LabelId) -> &mut Self {
        self.entry = Some(entry);
        self
    }

    pub fn block_mut(&mut self, label: LabelId) -> Option<&mut Block> {
        self.blocks.iter_mut().find(|block| block.label == label)
    }
}
