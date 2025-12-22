use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, stmt::block::BlockStmt},
    Parsable,
};

#[derive(Debug, PartialEq)]
pub enum ElseClause {
    If(Box<IfStmt>),
    Block(Box<BlockStmt>),
}

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<BlockStmt>,
    pub else_branch: Option<ElseClause>,
}

impl Parsable for IfStmt {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::If))?;

        let condition = parser.enclosed(TokenKind::LeftParen, TokenKind::RightParen, |parser| {
            Expr::parse(parser)
        })?;

        let then_branch = Box::new(BlockStmt::parse(parser)?);

        let else_branch = if parser.r#match(&[TokenKind::Keyword(KeywordKind::Else)]) {
            if parser.r#match(&[TokenKind::Keyword(KeywordKind::If)]) {
                Some(ElseClause::If(Box::new(IfStmt::parse(parser)?)))
            } else {
                Some(ElseClause::Block(Box::new(BlockStmt::parse(parser)?)))
            }
        } else {
            None
        };

        Ok(IfStmt {
            condition,
            then_branch,
            else_branch,
        })
    }
}

#[cfg(test)]
mod if_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            stmt::{
                block::BlockStmt,
                r#if::{ElseClause, IfStmt},
            },
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_if_stmt_without_else() {
        let source = "if (true) { }";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");

        let mut parser = Parser::new(&tokens);
        let if_stmt = IfStmt::parse(&mut parser).expect("Failed to parse");

        assert_eq!(
            if_stmt,
            IfStmt {
                condition: Expr::Primary(Primary::Literal(Literal::Boolean(true))),
                then_branch: Box::new(BlockStmt { statements: vec![] }),
                else_branch: None,
            }
        );
    }

    #[test]
    fn parses_if_stmt_with_else_block() {
        let source = "if (false) { } else { }";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");

        let mut parser = Parser::new(&tokens);
        let if_stmt = IfStmt::parse(&mut parser).expect("Failed to parse");

        assert_eq!(
            if_stmt,
            IfStmt {
                condition: Expr::Primary(Primary::Literal(Literal::Boolean(false))),
                then_branch: Box::new(BlockStmt { statements: vec![] }),
                else_branch: Some(ElseClause::Block(Box::new(BlockStmt {
                    statements: vec![]
                }))),
            }
        );
    }
}
