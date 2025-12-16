//! Expression AST nodes and primitives.
use crate::parser::{
    ast::{
        dfn::models::{ArgumentBodyDfn, ObjectDfn, ParameterBodyDfn},
        stmt::models::BlockStmt,
    },
    error::SpanSlice,
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
    #[production(span_slice: SpanSlice, operator: UnaryOperator, right: Expr)]
    Standard(StandardUnary),
    /// Pass-through for already fully parsed member expressions.
    #[production(span_slice: SpanSlice, expr: Expr)]
    Pass(PassUnary),
}

/// Root expression enum.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Expr {
    /// Leaf literal or primitive.
    #[production(span_slice: SpanSlice, primary: Primary)]
    Primary(Box<PrimaryExpr>),

    /// Unary operator expression.
    #[production(span_slice: SpanSlice, unary: Unary)]
    Unary(Box<UnaryExpr>),

    /// Function call expression.
    #[production(span_slice: SpanSlice, callee: Expr, argument_body: Option<ArgumentBodyDfn>)]
    Call(Box<CallExpr>),

    /// Member access chain.
    #[production(span_slice: SpanSlice, left: Expr, members: Vec<Expr>)]
    Member(Box<MemberExpr>),

    /// Binary operator expression.
    #[production(span_slice: SpanSlice, left: Expr, operator: Option<BinaryOperator>, right: Option<Expr>)]
    Binary(Box<BinaryExpr>),

    /// Assignment expression.
    #[production(span_slice: SpanSlice, left: Expr, right: Option<Expr>)]
    Assignment(Box<AssignmentExpr>),
}

/// Primary expression grouping literals and primitives.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Primary {
    /// Literal (string, number, boolean, none).
    #[production(span_slice: SpanSlice, literal: Literal)]
    Literal(LiteralPrimary),

    /// Primitive (object, grouping, identifier, lambda, path, context).
    #[production(span_slice: SpanSlice, primitive: Primitive)]
    Primitive(PrimitivePrimary),
}

/// Literal variants.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Literal {
    /// Numeric literal.
    #[production(span_slice: SpanSlice, value: f32)]
    Number(NumberLiteral),

    /// Boolean literal.
    #[production(span_slice: SpanSlice, value: bool)]
    Boolean(BooleanLiteral),

    /// String literal.
    #[production(span_slice: SpanSlice, value: String)]
    String(StringLiteral),

    /// `none` literal.
    #[production(span_slice: SpanSlice)]
    None(NoneLiteral),
}

/// Non-literal primitives (objects, identifiers, lambdas, paths, context).
#[derive(SyntaxTree, Debug, Clone)]
pub enum Primitive {
    /// Object literal primitive.
    #[production(span_slice: SpanSlice, object: ObjectDfn)]
    Object(ObjectPrimitive),

    /// Parenthesized grouping primitive.
    #[production(span_slice: SpanSlice, expr: Expr)]
    Grouping(GroupingPrimitive),

    /// Identifier reference.
    #[production(span_slice: SpanSlice, name: String)]
    Identifier(IdentifierPrimitive),

    /// Lambda primitive.
    #[production(span_slice: SpanSlice, parameters: ParameterBodyDfn, block_stmt: BlockStmt)]
    Lambda(LambdaPrimitive),

    /// Path (module-qualified identifier).
    #[production(span_slice: SpanSlice, identifiers: Vec<IdentifierPrimitive>)]
    Path(PathPrimitive),

    /// Current story context handle.
    #[production(span_slice: SpanSlice)]
    Context(ContextPrimitive),
}
