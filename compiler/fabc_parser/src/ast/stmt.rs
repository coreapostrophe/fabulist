use crate::ast::{expr::Expr, primitive::Primitive};

#[derive(Debug, PartialEq)]
pub enum ElseClause {
    If(Box<Stmt>),
    Block(Box<Stmt>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Block(Vec<Stmt>),
    Let {
        name: String,
        initializer: Expr,
    },
    Goto {
        path: Primitive,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<ElseClause>,
    },
    Function {
        name: String,
        parameters: Vec<String>,
        body: Box<Stmt>,
    },
}
