use pest::iterators::Pair;

use crate::{
    ast::expr::models::{Expr, IdentifierPrimitive, PathPrimitive},
    error::ParsingError,
    parser::Rule,
};

use super::models::{BlockStmt, ElseClause, GotoStmt, IfStmt, LetStmt, Stmt};

impl TryFrom<Pair<'_, Rule>> for ElseClause {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let mut inner = value.into_inner();

        if let Some(if_stmt) = inner.clone().find(|pair| pair.as_rule() == Rule::if_stmt) {
            Ok(ElseClause::If(IfStmt::try_from(if_stmt)?))
        } else if let Some(block_stmt) = inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Ok(ElseClause::Block(BlockStmt::try_from(block_stmt)?))
        } else {
            Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected an `if` or `block` statement",
            ))
        }
    }
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
    type Error = pest::error::Error<Rule>;
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
            _ => Err(ParsingError::map_custom_error(
                stmt_span.into(),
                "Invalid statement",
            )),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for BlockStmt {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let statements = value
            .into_inner()
            .map(Stmt::try_from)
            .collect::<Result<Vec<Stmt>, pest::error::Error<Rule>>>()?;

        Ok(BlockStmt {
            span: value_span.into(),
            statements,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for IfStmt {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let mut inner = value.into_inner();

        let condition = match inner.find(|pair| pair.as_node_tag() == Some("condition")) {
            Some(condition) => Expr::try_from(condition),
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected condition expression",
            )),
        }?;
        let block_stmt = match inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected block statement",
            )),
        }?;
        let else_stmt = match inner.find(|pair| pair.as_rule() == Rule::else_stmt) {
            Some(else_stmt) => Some(Box::new(ElseClause::try_from(else_stmt)?)),
            None => None,
        };

        Ok(IfStmt {
            span: value_span.into(),
            condition,
            block_stmt,
            else_stmt,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for LetStmt {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let mut inner = value.into_inner();

        let identifier = match inner.find(|pair| pair.as_rule() == Rule::identifier) {
            Some(identifier) => IdentifierPrimitive::try_from(identifier),
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected an identifier",
            )),
        }?;
        let value = match inner.find(|pair| pair.as_node_tag() == Some("value")) {
            Some(expression) => Expr::try_from(expression),
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected value expression",
            )),
        }?;

        Ok(LetStmt {
            span: value_span.into(),
            identifier,
            value,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for GotoStmt {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        let path = match value.into_inner().find(|pair| pair.as_rule() == Rule::path) {
            Some(path) => PathPrimitive::try_from(path),
            None => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Expected path expression",
            )),
        }?;

        Ok(GotoStmt {
            span: value_span.into(),
            path,
        })
    }
}
