//! Statement AST nodes (blocks, conditionals, bindings, jumps, and expression statements).
use crate::parser::{
    ast::expr::models::{Expr, IdentifierPrimitive, PathPrimitive},
    error::SpanSlice,
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
    #[production(span_slice: SpanSlice, statements: Vec<Stmt>)]
    Block(Box<BlockStmt>),

    /// `if` expression with optional `else` branch.
    #[production(span_slice: SpanSlice, condition: Expr, block_stmt: BlockStmt, else_stmt: Option<Box<ElseClause>>)]
    If(Box<IfStmt>),

    /// `let` binding statement.
    #[production(span_slice: SpanSlice, identifier: IdentifierPrimitive, value: Expr)]
    Let(Box<LetStmt>),

    /// Story navigation command (`goto module::part`).
    #[production(span_slice: SpanSlice, path: PathPrimitive)]
    Goto(Box<GotoStmt>),

    /// Standalone expression terminated with a semicolon.
    #[production(span_slice: SpanSlice, value: Expr)]
    Expr(Box<ExprStmt>),
}
