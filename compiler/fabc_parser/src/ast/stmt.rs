use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::stmt::{block::BlockStmt, expr::ExprStmt, goto::GotoStmt, r#if::IfStmt, r#let::LetStmt},
    error::Error,
    Parsable, Parser,
};

pub mod block;
pub mod expr;
pub mod goto;
pub mod r#if;
pub mod r#let;

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
}

impl Stmt {
    pub fn id(&self) -> usize {
        match self {
            Stmt::Expr(stmt) => stmt.id,
            Stmt::Block(stmt) => stmt.id,
            Stmt::Let(stmt) => stmt.id,
            Stmt::Goto(stmt) => stmt.id,
            Stmt::If(stmt) => stmt.id,
        }
    }
}

impl Parsable for Stmt {
    fn parse<'src, 'tok>(parser: &mut Parser<'src, 'tok>) -> Result<Self, Error> {
        match parser.peek() {
            TokenKind::Keyword(KeywordKind::Goto) => Ok(Stmt::Goto(GotoStmt::parse(parser)?)),
            TokenKind::Keyword(KeywordKind::If) => Ok(Stmt::If(IfStmt::parse(parser)?)),
            TokenKind::LeftBrace => Ok(Stmt::Block(BlockStmt::parse(parser)?)),
            TokenKind::Keyword(KeywordKind::Let) => Ok(Stmt::Let(LetStmt::parse(parser)?)),
            _ => Ok(Stmt::Expr(ExprStmt::parse(parser)?)),
        }
    }
}
