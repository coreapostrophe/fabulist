use std::collections::BTreeMap;

use super::FunctionId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Literal {
    Number(f64),
    Boolean(bool),
    String(String),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BinaryOperator {
    EqualEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UnaryOperator {
    Not,
    Negate,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MemberSegment {
    Key(String),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    StoryReference(String),
    Context,
    Object(BTreeMap<String, Expr>),
    Closure(FunctionId),
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    MemberAccess {
        base: Box<Expr>,
        members: Vec<MemberSegment>,
    },
    Assignment {
        target: Box<Expr>,
        value: Box<Expr>,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
}
