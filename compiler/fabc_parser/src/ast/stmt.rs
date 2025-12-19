use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{
    ast::stmt::{
        block::BlockStmt, expr::ExprStmt, function::FunctionStmt, goto::GotoStmt,
        module::ModuleStmt, r#if::IfStmt, r#let::LetStmt,
    },
    error::Error,
    Parsable, Parser,
};

pub mod block;
pub mod expr;
pub mod function;
pub mod goto;
pub mod r#if;
pub mod r#let;
pub mod module;

#[derive(Debug, PartialEq)]
pub enum ElseClause {
    If(Box<Stmt>),
    Block(Box<Stmt>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(ExprStmt),
    Block(BlockStmt),
    Let(LetStmt),
    Goto(GotoStmt),
    If(IfStmt),
    Function(FunctionStmt),
    Module(ModuleStmt),
}

impl Parsable for Stmt {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        if parser.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }

        match parser.peek() {
            Token::Keyword(KeywordKind::Fn) => Ok(Stmt::Function(FunctionStmt::parse(parser)?)),
            Token::Keyword(KeywordKind::Goto) => Ok(Stmt::Goto(GotoStmt::parse(parser)?)),
            Token::Keyword(KeywordKind::If) => Ok(Stmt::If(IfStmt::parse(parser)?)),
            Token::LeftBrace => Ok(Stmt::Block(BlockStmt::parse(parser)?)),
            Token::Keyword(KeywordKind::Let) => Ok(Stmt::Let(LetStmt::parse(parser)?)),
            Token::Keyword(KeywordKind::Module) => Ok(Stmt::Module(ModuleStmt::parse(parser)?)),
            _ => Ok(Stmt::Expr(ExprStmt::parse(parser)?)),
        }
    }
}
