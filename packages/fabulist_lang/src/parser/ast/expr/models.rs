//! Expression AST nodes and primitives.
use crate::{
    error::OwnedSpan,
    parser::ast::{
        dfn::models::{ArgumentBodyDfn, ObjectDfn, ParameterBodyDfn},
        stmt::models::BlockStmt,
    },
};
use fabulist_derive::SyntaxTree;

/// Binary operators supported by the language.
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    /// `/`
    Divide,
    /// `*`
    Multiply,
    /// `+`
    Addition,
    /// `-`
    Subtraction,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterEqual,
    /// `<`
    LessThan,
    /// `<=`
    LessEqual,
    /// `==`
    EqualEqual,
    /// `!=`
    NotEqual,
    /// `&&`
    And,
    /// `||`
    Or,
}

/// Unary operators supported by the language.
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    /// Numeric negation (`-value`).
    Negation,
    /// Logical not (`!value`).
    Not,
}

/// High-level unary wrapper (either a real operator or a pass-through of the operand).
#[derive(SyntaxTree, Debug, Clone)]
pub enum Unary {
    /// Unary operator applied to an expression.
    #[production(span: OwnedSpan, operator: UnaryOperator, right: Expr)]
    Standard(StandardUnary),
    /// Pass-through for already fully parsed member expressions.
    #[production(span: OwnedSpan, expr: Expr)]
    Pass(PassUnary),
}

/// Root expression enum.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Expr {
    /// Leaf literal or primitive.
    #[production(span: OwnedSpan, primary: Primary)]
    Primary(Box<PrimaryExpr>),

    /// Unary operator expression.
    #[production(span: OwnedSpan, unary: Unary)]
    Unary(Box<UnaryExpr>),

    /// Function call expression.
    #[production(span: OwnedSpan, callee: Expr, argument_body: Option<ArgumentBodyDfn>)]
    Call(Box<CallExpr>),

    /// Member access chain.
    #[production(span: OwnedSpan, left: Expr, members: Vec<Expr>)]
    Member(Box<MemberExpr>),

    /// Binary operator expression.
    #[production(span: OwnedSpan, left: Expr, operator: Option<BinaryOperator>, right: Option<Expr>)]
    Binary(Box<BinaryExpr>),

    /// Assignment expression.
    #[production(span: OwnedSpan, left: Expr, right: Option<Expr>)]
    Assignment(Box<AssignmentExpr>),
}

/// Primary expression grouping literals and primitives.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Primary {
    /// Literal (string, number, boolean, none).
    #[production(span: OwnedSpan, literal: Literal)]
    Literal(LiteralPrimary),

    /// Primitive (object, grouping, identifier, lambda, path, context).
    #[production(span: OwnedSpan, primitive: Primitive)]
    Primitive(PrimitivePrimary),
}

/// Literal variants.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Literal {
    /// Numeric literal.
    #[production(span: OwnedSpan, value: f32)]
    Number(NumberLiteral),

    /// Boolean literal.
    #[production(span: OwnedSpan, value: bool)]
    Boolean(BooleanLiteral),

    /// String literal.
    #[production(span: OwnedSpan, value: String)]
    String(StringLiteral),

    /// `none` literal.
    #[production(span: OwnedSpan)]
    None(NoneLiteral),
}

/// Non-literal primitives (objects, identifiers, lambdas, paths, context).
#[derive(SyntaxTree, Debug, Clone)]
pub enum Primitive {
    /// Object literal primitive.
    #[production(span: OwnedSpan, object: ObjectDfn)]
    Object(ObjectPrimitive),

    /// Parenthesized grouping primitive.
    #[production(span: OwnedSpan, expr: Expr)]
    Grouping(GroupingPrimitive),

    /// Identifier reference.
    #[production(span: OwnedSpan, name: String)]
    Identifier(IdentifierPrimitive),

    /// Lambda primitive.
    #[production(span: OwnedSpan, parameters: ParameterBodyDfn, block_stmt: BlockStmt)]
    Lambda(LambdaPrimitive),

    /// Path (module-qualified identifier).
    #[production(span: OwnedSpan, identifiers: Vec<IdentifierPrimitive>)]
    Path(PathPrimitive),

    /// Current story context handle.
    #[production(span: OwnedSpan)]
    Context(ContextPrimitive),
}
