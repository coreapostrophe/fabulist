use fabc_error::Error;
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{
        stmt::{
            block::BlockStmt, expr::ExprStmt, goto::GotoStmt, r#if::IfStmt, r#let::LetStmt,
            r#return::ReturnStmt,
        },
        NodeInfo,
    },
    Parsable, Parser,
};

pub mod block;
pub mod expr;
pub mod goto;
pub mod r#if;
pub mod r#let;
pub mod r#return;

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
    Return(ReturnStmt),
}

impl Stmt {
    pub fn info(&self) -> &NodeInfo {
        match self {
            Stmt::Expr(stmt) => &stmt.info,
            Stmt::Block(stmt) => &stmt.info,
            Stmt::Let(stmt) => &stmt.info,
            Stmt::Goto(stmt) => &stmt.info,
            Stmt::If(stmt) => &stmt.info,
            Stmt::Return(stmt) => &stmt.info,
        }
    }
}

impl Parsable for Stmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        match parser.peek() {
            TokenKind::Keyword(KeywordKind::Goto) => Ok(Stmt::Goto(GotoStmt::parse(parser)?)),
            TokenKind::Keyword(KeywordKind::If) => Ok(Stmt::If(IfStmt::parse(parser)?)),
            TokenKind::LeftBrace => Ok(Stmt::Block(BlockStmt::parse(parser)?)),
            TokenKind::Keyword(KeywordKind::Let) => Ok(Stmt::Let(LetStmt::parse(parser)?)),
            TokenKind::Keyword(KeywordKind::Return) => Ok(Stmt::Return(ReturnStmt::parse(parser)?)),
            _ => Ok(Stmt::Expr(ExprStmt::parse(parser)?)),
        }
    }
}
