use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{stmt::Stmt, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct BlockStmt {
    pub info: NodeInfo,
    pub first_return: Option<usize>,
    pub statements: Vec<Stmt>,
}

impl Parsable for BlockStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        let mut first_return = None;
        let statements =
            parser.enclosed(TokenKind::LeftBrace, TokenKind::RightBrace, |parser| {
                let mut stmt_vec = Vec::new();
                let mut idx_count = 0;

                while parser.peek() != &TokenKind::RightBrace && !parser.is_terminated() {
                    let stmt = Stmt::parse(parser);
                    match stmt {
                        Ok(stmt) => {
                            if let Stmt::Return(_) = &stmt {
                                if first_return.is_none() {
                                    first_return = Some(idx_count);
                                }
                            }
                            stmt_vec.push(stmt);
                        }
                        Err(err) => parser.push_error(err),
                    }
                    idx_count += 1;
                }
                Ok(stmt_vec)
            })?;
        let end_span = parser.end_span();

        Ok(BlockStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            first_return,
            statements,
        })
    }
}

#[cfg(test)]
mod block_stmt_tests {
    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            stmt::{block::BlockStmt, r#let::LetStmt, Stmt},
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_block_statements() {
        let source = "{ let a = 1; let b = 2; }";
        let tokens = Lexer::tokenize(source);
        let block_stmt =
            Parser::parse_ast::<BlockStmt>(&tokens).expect("Failed to parse block statement");

        let expected = BlockStmt {
            info: NodeInfo {
                id: 4,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 25))),
            },
            first_return: None,
            statements: vec![
                Stmt::Let(LetStmt {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(1, 3), LineCol::new(1, 12))),
                    },
                    name: "a".to_string(),
                    initializer: Expr::Primary {
                        info: NodeInfo {
                            id: 0,
                            span: Span::from((LineCol::new(1, 11), LineCol::new(1, 11))),
                        },
                        value: Primary::Literal(Literal::Number(1.0)),
                    },
                }),
                Stmt::Let(LetStmt {
                    info: NodeInfo {
                        id: 3,
                        span: Span::from((LineCol::new(1, 14), LineCol::new(1, 23))),
                    },
                    name: "b".to_string(),
                    initializer: Expr::Primary {
                        info: NodeInfo {
                            id: 2,
                            span: Span::from((LineCol::new(1, 22), LineCol::new(1, 22))),
                        },
                        value: Primary::Literal(Literal::Number(2.0)),
                    },
                }),
            ],
        };

        assert_eq!(block_stmt, expected);
    }
}
