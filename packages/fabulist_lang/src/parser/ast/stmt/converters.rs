//! Converters from pest parse pairs into statement AST nodes.
use pest::iterators::Pair;

use crate::parser::{
    ast::{
        expr::models::{Expr, IdentifierPrimitive, PathPrimitive},
        stmt::models::ExprStmt,
    },
    error::{ExtractSpanSlice, ParserError},
    Rule,
};

use super::models::{BlockStmt, ElseClause, GotoStmt, IfStmt, LetStmt, Stmt};

impl TryFrom<Pair<'_, Rule>> for ElseClause {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        if let Some(if_stmt) = inner.clone().find(|pair| pair.as_rule() == Rule::if_stmt) {
            Ok(ElseClause::If(IfStmt::try_from(if_stmt)?))
        } else if let Some(block_stmt) = inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Ok(ElseClause::Block(BlockStmt::try_from(block_stmt)?))
        } else {
            Err(ParserError::ExpectedSymbol {
                expected: "an `if` or `block` statement".to_string(),
                span_slice: value_span_slice,
            })
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

impl From<ExprStmt> for Stmt {
    fn from(value: ExprStmt) -> Self {
        Self::Expr(Box::new(value))
    }
}

impl TryFrom<Pair<'_, Rule>> for Stmt {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::statement => match value.into_inner().next() {
                Some(inner) => Ok(Stmt::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::block_stmt => Ok(BlockStmt::try_from(value)?.into()),
            Rule::if_stmt => Ok(IfStmt::try_from(value)?.into()),
            Rule::let_stmt => Ok(LetStmt::try_from(value)?.into()),
            Rule::goto_stmt => Ok(GotoStmt::try_from(value)?.into()),
            Rule::expression_stmt => Ok(ExprStmt::try_from(value)?.into()),
            _ => Err(ParserError::InvalidStatement(value.extract_span_slice())),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ExprStmt {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let expr = match inner.find(|pair| pair.as_node_tag() == Some("value")) {
            Some(expression) => Expr::try_from(expression),
            None => Err(ParserError::ExpectedSymbol {
                expected: "expression".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        Ok(ExprStmt {
            span_slice: value_span_slice,
            value: expr,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for BlockStmt {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        let statements = value
            .into_inner()
            .map(Stmt::try_from)
            .collect::<Result<Vec<Stmt>, ParserError>>()?;

        Ok(BlockStmt {
            span_slice: value_span_slice,
            statements,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for IfStmt {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let condition = match inner.find(|pair| pair.as_node_tag() == Some("condition")) {
            Some(condition) => Expr::try_from(condition),
            None => Err(ParserError::ExpectedSymbol {
                expected: "condition expression".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        let block_stmt = match inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(ParserError::ExpectedSymbol {
                expected: "block statement".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        let else_stmt = match inner.find(|pair| pair.as_rule() == Rule::else_stmt) {
            Some(else_stmt) => Some(Box::new(ElseClause::try_from(else_stmt)?)),
            None => None,
        };

        Ok(IfStmt {
            span_slice: value_span_slice,
            condition,
            block_stmt,
            else_stmt,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for LetStmt {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();
        let mut inner = value.into_inner();

        let identifier = match inner.find(|pair| pair.as_rule() == Rule::identifier) {
            Some(identifier) => IdentifierPrimitive::try_from(identifier),
            None => Err(ParserError::ExpectedSymbol {
                expected: "identifier".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        let value = match inner.find(|pair| pair.as_node_tag() == Some("value")) {
            Some(expression) => Expr::try_from(expression),
            None => Err(ParserError::ExpectedSymbol {
                expected: "value expression".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        Ok(LetStmt {
            span_slice: value_span_slice,
            identifier,
            value,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for GotoStmt {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        let path = match value.into_inner().find(|pair| pair.as_rule() == Rule::path) {
            Some(path) => PathPrimitive::try_from(path),
            None => Err(ParserError::ExpectedSymbol {
                expected: "path expression".to_string(),
                span_slice: value_span_slice.clone(),
            }),
        }?;

        Ok(GotoStmt {
            span_slice: value_span_slice,
            path,
        })
    }
}

#[cfg(test)]
mod stmt_converters_tests {
    use crate::{parser::ast::AstTestHelper, parser::Rule};

    use super::*;

    #[test]
    fn parses_block_stmt() {
        let test_helper = AstTestHelper::<BlockStmt>::new(Rule::block_stmt, "BlockStmt");
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
        let test_helper = AstTestHelper::<IfStmt>::new(Rule::if_stmt, "IfStmt");
        test_helper.assert_parse("if true {}");
        test_helper.assert_parse("if true {} else {}");
    }

    #[test]
    fn parses_let_stmt() {
        let test_helper = AstTestHelper::<LetStmt>::new(Rule::let_stmt, "LetStmt");
        test_helper.assert_parse("let key = \"value\";");
    }

    #[test]
    fn parses_goto_stmt() {
        let test_helper = AstTestHelper::<GotoStmt>::new(Rule::goto_stmt, "GotoStmt");
        test_helper.assert_parse("goto module_1::part_1;");
    }
}
