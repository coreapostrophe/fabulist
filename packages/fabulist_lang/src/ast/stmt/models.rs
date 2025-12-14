use crate::{
    ast::expr::models::{Expr, IdentifierPrimitive, PathPrimitive},
    error::OwnedSpan,
};
use fabulist_derive::SyntaxTree;

#[derive(Debug, Clone)]
pub enum ElseClause {
    If(IfStmt),
    Block(BlockStmt),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Stmt {
    #[production(span: OwnedSpan, statements: Vec<Stmt>)]
    Block(Box<BlockStmt>),

    #[production(span: OwnedSpan, condition: Expr, block_stmt: BlockStmt, else_stmt: Option<Box<ElseClause>>)]
    If(Box<IfStmt>),

    #[production(span: OwnedSpan, identifier: IdentifierPrimitive, value: Expr)]
    Let(Box<LetStmt>),

    #[production(span: OwnedSpan, path: PathPrimitive)]
    Goto(Box<GotoStmt>),
}
