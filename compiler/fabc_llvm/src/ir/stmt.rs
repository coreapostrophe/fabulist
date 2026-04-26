use super::Expr;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Stmt {
    Expr(Expr),
    Block(Block),
    Let {
        name: String,
        initializer: Expr,
    },
    Goto(Expr),
    If {
        condition: Expr,
        then_branch: Block,
        else_branch: Option<Box<Stmt>>,
    },
    Return(Option<Expr>),
}
