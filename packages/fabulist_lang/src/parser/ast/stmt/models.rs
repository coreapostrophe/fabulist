//! Statement AST nodes (blocks, conditionals, bindings, jumps, and expression statements).
use crate::{
    error::SpanSlice,
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
    #[production(span: SpanSlice, statements: Vec<Stmt>)]
    Block(Box<BlockStmt>),

    /// `if` expression with optional `else` branch.
    #[production(span: SpanSlice, condition: Expr, block_stmt: BlockStmt, else_stmt: Option<Box<ElseClause>>)]
    If(Box<IfStmt>),

    /// `let` binding statement.
    #[production(span: SpanSlice, identifier: IdentifierPrimitive, value: Expr)]
    Let(Box<LetStmt>),

    /// Story navigation command (`goto module::part`).
    #[production(span: SpanSlice, path: PathPrimitive)]
    Goto(Box<GotoStmt>),

    /// Standalone expression terminated with a semicolon.
    #[production(span: SpanSlice, value: Expr)]
    Expr(Box<ExprStmt>),
}
