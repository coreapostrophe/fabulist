use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, stmt::block::BlockStmt},
    error::Error,
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub enum ElseClause {
    If(Box<IfStmt>),
    Block(Box<BlockStmt>),
}

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    pub id: usize,
    pub condition: Expr,
    pub then_branch: Box<BlockStmt>,
    pub else_branch: Option<ElseClause>,
}

impl Parsable for IfStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
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
            id: parser.assign_id(),
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
        Parser,
    };

    #[test]
    fn parses_if_stmt_without_else() {
        let source = "if (true) { }";
        let tokens = Lexer::tokenize(source);
        let if_stmt = Parser::parse_ast::<IfStmt>(&tokens).expect("Failed to parse if statement");

        assert_eq!(
            if_stmt,
            IfStmt {
                id: 2,
                condition: Expr::Primary {
                    id: 0,
                    value: Primary::Literal(Literal::Boolean(true)),
                },
                then_branch: Box::new(BlockStmt {
                    id: 1,
                    statements: vec![]
                }),
                else_branch: None,
            }
        );
    }

    #[test]
    fn parses_if_stmt_with_else_block() {
        let source = "if (false) { } else { }";
        let tokens = Lexer::tokenize(source);
        let if_stmt = Parser::parse_ast::<IfStmt>(&tokens).expect("Failed to parse if statement");

        assert_eq!(
            if_stmt,
            IfStmt {
                id: 3,
                condition: Expr::Primary {
                    id: 0,
                    value: Primary::Literal(Literal::Boolean(false)),
                },
                then_branch: Box::new(BlockStmt {
                    id: 1,
                    statements: vec![]
                }),
                else_branch: Some(ElseClause::Block(Box::new(BlockStmt {
                    id: 2,
                    statements: vec![]
                }))),
            }
        );
    }
}
