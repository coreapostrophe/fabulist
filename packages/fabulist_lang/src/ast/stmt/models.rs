use crate::ast::expr::models::{Expr, IdentifierPrimitive, PathPrimitive};
use fabulist_derive::SyntaxTree;

#[derive(Debug, Clone)]
pub enum ElseClause {
    If(IfStmt),
    Block(BlockStmt),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Stmt {
    #[production(statements: Vec<Stmt>)]
    Block(Box<BlockStmt>),

    #[production(condition: Expr, block_stmt: BlockStmt, else_stmt: Option<Box<ElseClause>>)]
    If(Box<IfStmt>),

    #[production(identifier: IdentifierPrimitive, value: Expr)]
    Let(Box<LetStmt>),

    #[production(path: PathPrimitive)]
    Goto(Box<GotoStmt>),
}

#[cfg(test)]
mod stmt_tests {
    use crate::{ast::ParserTestHelper, parser::Rule};

    use super::*;

    #[test]
    fn parses_block_stmt() {
        let test_helper = ParserTestHelper::<BlockStmt>::new(Rule::block_stmt, "BlockStmt");
        test_helper.assert_parse(
            r#"{
                let key = "value";
                goto module_1::part_1;
                if true {} else if true {} else {}
            }"#,
        );
    }

    #[test]
    fn parses_if_stmt() {
        let test_helper = ParserTestHelper::<IfStmt>::new(Rule::if_stmt, "IfStmt");
        test_helper.assert_parse("if true {}");
        test_helper.assert_parse("if true {} else {}");
    }

    #[test]
    fn parses_let_stmt() {
        let test_helper = ParserTestHelper::<LetStmt>::new(Rule::let_stmt, "LetStmt");
        test_helper.assert_parse("let key = \"value\";");
    }

    #[test]
    fn parses_goto_stmt() {
        let test_helper = ParserTestHelper::<GotoStmt>::new(Rule::goto_stmt, "GotoStmt");
        test_helper.assert_parse("goto module_1::part_1;");
    }
}
