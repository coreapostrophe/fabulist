//! Statement AST nodes (blocks, conditionals, bindings, jumps, and expression statements).
use crate::{
    error::OwnedSpan,
    parser::ast::expr::models::{Expr, IdentifierPrimitive, PathPrimitive},
};
use fabulist_derive::SyntaxTree;

/// `else` clause that can chain another `if` or a block.
#[derive(Debug, Clone)]
pub enum ElseClause {
    /// Chained `else if` clause.
    If(IfStmt),
    /// Trailing `else { ... }` block.
    Block(BlockStmt),
}

/// Statement variants used by the interpreter.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Stmt {
    /// `{ ... }` scoped block of statements.
    #[production(span: OwnedSpan, statements: Vec<Stmt>)]
    Block(Box<BlockStmt>),

    /// `if` expression with optional `else` branch.
    #[production(span: OwnedSpan, condition: Expr, block_stmt: BlockStmt, else_stmt: Option<Box<ElseClause>>)]
    If(Box<IfStmt>),

    /// `let` binding statement.
    #[production(span: OwnedSpan, identifier: IdentifierPrimitive, value: Expr)]
    Let(Box<LetStmt>),

    /// Story navigation command (`goto module::part`).
    #[production(span: OwnedSpan, path: PathPrimitive)]
    Goto(Box<GotoStmt>),

    /// Standalone expression terminated with a semicolon.
    #[production(span: OwnedSpan, value: Expr)]
    Expr(Box<ExprStmt>),
}
