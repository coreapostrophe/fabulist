use pest::iterators::Pair;

use crate::parser::Rule;

use self::{block_stmt::BlockStmt, goto_stmt::GotoStmt, if_stmt::IfStmt, let_stmt::LetStmt};

use super::Error;

pub mod block_stmt;
pub mod else_stmt;
pub mod goto_stmt;
pub mod if_stmt;
pub mod let_stmt;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Box<BlockStmt>),
    If(Box<IfStmt>),
    Let(Box<LetStmt>),
    Goto(Box<GotoStmt>),
}

impl From<BlockStmt> for Stmt {
    fn from(value: BlockStmt) -> Self {
        Self::Block(Box::new(value))
    }
}

impl From<IfStmt> for Stmt {
    fn from(value: IfStmt) -> Self {
        Self::If(Box::new(value))
    }
}

impl From<LetStmt> for Stmt {
    fn from(value: LetStmt) -> Self {
        Self::Let(Box::new(value))
    }
}

impl From<GotoStmt> for Stmt {
    fn from(value: GotoStmt) -> Self {
        Self::Goto(Box::new(value))
    }
}

impl TryFrom<Pair<'_, Rule>> for Stmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let stmt_span = value.as_span();
        match value.as_rule() {
            Rule::statement => match value.into_inner().next() {
                Some(inner) => Ok(Stmt::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::block_stmt => Ok(BlockStmt::try_from(value)?.into()),
            Rule::if_stmt => Ok(IfStmt::try_from(value)?.into()),
            Rule::let_stmt => Ok(LetStmt::try_from(value)?.into()),
            Rule::goto_stmt => Ok(GotoStmt::try_from(value)?.into()),
            _ => Err(Error::map_span(stmt_span, "Invalid statement")),
        }
    }
}
