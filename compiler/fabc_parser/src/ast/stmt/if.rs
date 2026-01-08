use fabc_error::{Error, Span};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, stmt::block::BlockStmt, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub enum ElseClause {
    If(Box<IfStmt>),
    Block(Box<BlockStmt>),
}

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    pub info: NodeInfo,
    pub condition: Expr,
    pub then_branch: Box<BlockStmt>,
    pub else_branch: Option<ElseClause>,
}

impl Parsable for IfStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();

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

        let end_span = parser.end_span();

        Ok(IfStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            condition,
            then_branch,
            else_branch,
        })
    }
}

#[cfg(test)]
mod if_stmt_tests {
    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            stmt::{
                block::BlockStmt,
                r#if::{ElseClause, IfStmt},
            },
            NodeInfo,
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
                info: NodeInfo {
                    id: 2,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 13)))
                },
                condition: Expr::Primary {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 5), LineCol::new(1, 8)))
                    },
                    value: Primary::Literal(Literal::Boolean(true)),
                },
                then_branch: Box::new(BlockStmt {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(1, 11), LineCol::new(1, 13)))
                    },
                    last_return: None,
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
                info: NodeInfo {
                    id: 3,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 23)))
                },
                condition: Expr::Primary {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 5), LineCol::new(1, 9)))
                    },
                    value: Primary::Literal(Literal::Boolean(false)),
                },
                then_branch: Box::new(BlockStmt {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(1, 12), LineCol::new(1, 14)))
                    },
                    last_return: None,
                    statements: vec![]
                }),
                else_branch: Some(ElseClause::Block(Box::new(BlockStmt {
                    info: NodeInfo {
                        id: 2,
                        span: Span::from((LineCol::new(1, 21), LineCol::new(1, 23)))
                    },
                    last_return: None,
                    statements: vec![]
                }))),
            }
        );
    }
}
