use crate::ast::{
    dfn::{ArgumentBodyDfn, ObjectDfn, ParameterBodyDfn},
    stmt::BlockStmt,
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
    #[production(operator: UnaryOperator, right: Expr)]
    Standard(StandardUnary),
    #[production(expr: Expr)]
    Pass(PassUnary),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Expr {
    #[production(primary: Primary)]
    Primary(Box<PrimaryExpr>),

    #[production(unary: Unary)]
    Unary(Box<UnaryExpr>),

    #[production(callee: Expr, argument_body: Option<ArgumentBodyDfn>)]
    Call(Box<CallExpr>),

    #[production(left: Expr, members: Vec<Expr>)]
    Member(Box<MemberExpr>),

    #[production(left: Expr, operator: Option<BinaryOperator>, right: Option<Expr>)]
    Binary(Box<BinaryExpr>),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Primary {
    #[production(literal: Literal)]
    Literal(LiteralPrimary),

    #[production(primitive: Primitive)]
    Primitive(PrimitivePrimary),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Literal {
    #[production(value: f32)]
    Number(NumberLiteral),

    #[production(value: bool)]
    Boolean(BooleanLiteral),

    #[production(value: String)]
    String(StringLiteral),

    #[production]
    None(NoneLiteral),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Primitive {
    #[production(object: ObjectDfn)]
    Object(ObjectPrimitive),

    #[production(expr: Expr)]
    Grouping(GroupingPrimitive),

    #[production(name: String)]
    Identifier(IdentifierPrimitive),

    #[production(parameters: ParameterBodyDfn, block_stmt: BlockStmt)]
    Lambda(LambdaPrimitive),

    #[production(identifiers: Vec<IdentifierPrimitive>)]
    Path(PathPrimitive),

    #[production]
    Context(ContextPrimitive),
}

#[cfg(test)]
mod expr_tests {
    use crate::{ast::ParserTestHelper, parser::Rule};

    use super::*;

    #[test]
    fn parses_unary_expr() {
        let test_helper = ParserTestHelper::<UnaryExpr>::new(Rule::unary_expr, "UnaryExpr");
        test_helper.assert_parse("!5");
        test_helper.assert_parse("!(true)");
        test_helper.assert_parse("!!!ident");
        test_helper.assert_parse("-\"num\"");
    }

    #[test]
    fn parses_call_expr() {
        let test_helper = ParserTestHelper::<CallExpr>::new(Rule::call_expr, "CallExpr");
        test_helper.assert_parse("test()");
        test_helper.assert_parse("5()");
        test_helper.assert_parse("\"Yo\"()");
        test_helper.assert_parse("false()");
    }

    #[test]
    fn parses_member_expr() {
        let test_helper = ParserTestHelper::<MemberExpr>::new(Rule::member_expr, "MemberExpr");
        test_helper.assert_parse("ident.fun().fun()");
        test_helper.assert_parse("ident.fun(arg1, arg2).fun(arg1, arg2)");
    }

    #[test]
    fn parses_binary_expr() {
        let test_helper = ParserTestHelper::<BinaryExpr>::new(Rule::logical_expr, "BinaryExpr");
        test_helper.assert_parse("5 + 2");
        test_helper.assert_parse("5/ 2");
        test_helper.assert_parse("5 *2");
        test_helper.assert_parse("5== 2");
    }

    #[test]
    fn parses_primaries() {
        let test_helper = ParserTestHelper::<Primary>::new(Rule::primary_expr, "PrimaryExpr");
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
        let test_helper = ParserTestHelper::<Literal>::new(Rule::literal_expr, "LiteralExpr");
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
        let test_helper = ParserTestHelper::<Primitive>::new(Rule::primitive_expr, "PrimitiveExpr");
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
