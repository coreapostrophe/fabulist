use crate::{
    ast::{
        dfn::models::{ArgumentBodyDfn, ObjectDfn, ParameterBodyDfn},
        stmt::models::BlockStmt,
    },
    error::OwnedSpan,
};
use fabulist_derive::SyntaxTree;

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Divide,
    Multiply,
    Addition,
    Subtraction,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
    EqualEqual,
    NotEqual,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negation,
    Not,
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Unary {
    #[production(span: OwnedSpan, operator: UnaryOperator, right: Expr)]
    Standard(StandardUnary),
    #[production(span: OwnedSpan, expr: Expr)]
    Pass(PassUnary),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Expr {
    #[production(span: OwnedSpan, primary: Primary)]
    Primary(Box<PrimaryExpr>),

    #[production(span: OwnedSpan, unary: Unary)]
    Unary(Box<UnaryExpr>),

    #[production(span: OwnedSpan, callee: Expr, argument_body: Option<ArgumentBodyDfn>)]
    Call(Box<CallExpr>),

    #[production(span: OwnedSpan, left: Expr, members: Vec<Expr>)]
    Member(Box<MemberExpr>),

    #[production(span: OwnedSpan, left: Expr, operator: Option<BinaryOperator>, right: Option<Expr>)]
    Binary(Box<BinaryExpr>),

    #[production(span: OwnedSpan, left: Expr, right: Option<Expr>)]
    Assignment(Box<AssignmentExpr>),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Primary {
    #[production(span: OwnedSpan, literal: Literal)]
    Literal(LiteralPrimary),

    #[production(span: OwnedSpan, primitive: Primitive)]
    Primitive(PrimitivePrimary),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Literal {
    #[production(span: OwnedSpan, value: f32)]
    Number(NumberLiteral),

    #[production(span: OwnedSpan, value: bool)]
    Boolean(BooleanLiteral),

    #[production(span: OwnedSpan, value: String)]
    String(StringLiteral),

    #[production(span: OwnedSpan)]
    None(NoneLiteral),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Primitive {
    #[production(span: OwnedSpan, object: ObjectDfn)]
    Object(ObjectPrimitive),

    #[production(span: OwnedSpan, expr: Expr)]
    Grouping(GroupingPrimitive),

    #[production(span: OwnedSpan, name: String)]
    Identifier(IdentifierPrimitive),

    #[production(span: OwnedSpan, parameters: ParameterBodyDfn, block_stmt: BlockStmt)]
    Lambda(LambdaPrimitive),

    #[production(span: OwnedSpan, identifiers: Vec<IdentifierPrimitive>)]
    Path(PathPrimitive),

    #[production(span: OwnedSpan)]
    Context(ContextPrimitive),
}