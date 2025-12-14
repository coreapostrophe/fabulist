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

#[cfg(test)]
mod expr_tests {
    use crate::{ast::AstTestHelper, parser::Rule};

    use super::*;

    #[test]
    fn parses_unary_expr() {
        let test_helper = AstTestHelper::<UnaryExpr>::new(Rule::unary_expr, "UnaryExpr");
        test_helper.assert_parse("!5");
        test_helper.assert_parse("!(true)");
        test_helper.assert_parse("!!!ident");
        test_helper.assert_parse("-\"num\"");
    }

    #[test]
    fn parses_call_expr() {
        let test_helper = AstTestHelper::<CallExpr>::new(Rule::call_expr, "CallExpr");
        test_helper.assert_parse("test()");
        test_helper.assert_parse("5()");
        test_helper.assert_parse("\"Yo\"()");
        test_helper.assert_parse("false()");
    }

    #[test]
    fn parses_member_expr() {
        let test_helper = AstTestHelper::<MemberExpr>::new(Rule::member_expr, "MemberExpr");
        test_helper.assert_parse("ident.fun().fun()");
        test_helper.assert_parse("ident.fun(arg1, arg2).fun(arg1, arg2)");
    }

    #[test]
    fn parses_binary_expr() {
        let test_helper = AstTestHelper::<BinaryExpr>::new(Rule::logical_expr, "BinaryExpr");
        test_helper.assert_parse("5 + 2");
        test_helper.assert_parse("5/ 2");
        test_helper.assert_parse("5 *2");
        test_helper.assert_parse("5== 2");
    }

    #[test]
    fn parses_assignment_expr() {
        let test_helper =
            AstTestHelper::<AssignmentExpr>::new(Rule::assignment_expr, "AssignmentExpr");
        test_helper.assert_parse("a = 5");
        test_helper.assert_parse("b = a + 2");
    }

    #[test]
    fn parses_primaries() {
        let test_helper = AstTestHelper::<Primary>::new(Rule::primary_expr, "PrimaryExpr");
        test_helper.assert_parse("\"string\"");
        test_helper.assert_parse(r##"r"raw string""##);
        test_helper.assert_parse("2");
        test_helper.assert_parse("2.5");
        test_helper.assert_parse("none");
        test_helper.assert_parse("identifier");
        test_helper.assert_parse("r#none");
        test_helper.assert_parse("path::path_2::path_3");
        test_helper.assert_parse(r#"{"string": "string", "number": 5}"#);
    }

    #[test]
    fn parses_literal_expr() {
        let test_helper = AstTestHelper::<Literal>::new(Rule::literal_expr, "LiteralExpr");
        test_helper.assert_parse("\"string\"");
        test_helper.assert_parse("r#\"raw string\"#");
        test_helper.assert_parse("5");
        test_helper.assert_parse("5.52252");
        test_helper.assert_parse("none");
        test_helper.assert_parse("true");
        test_helper.assert_parse("false");
    }

    #[test]
    fn parses_primitive_expr() {
        let test_helper = AstTestHelper::<Primitive>::new(Rule::primitive_expr, "PrimitiveExpr");
        test_helper.assert_parse("ident");
        test_helper.assert_parse("r#module");
        test_helper.assert_parse("(ident)");
        test_helper.assert_parse("path::path2::path3");
        test_helper.assert_parse("{ \"key\": 5 }");
        test_helper.assert_parse("() => { goto module_1::part_1; }");
        test_helper.assert_parse("(param1, param2) => { let a = 5; }");
        test_helper.assert_parse("context");
    }
}
