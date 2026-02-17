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
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::stmt::block::BlockStmt, Parser};

    #[test]
    fn parses_block_statements() {
        let block_stmt = Parser::parse_ast_str::<BlockStmt>("{ let a = 1; let b = 2; }")
            .expect("Failed to parse block statement");

        assert_debug_snapshot!(block_stmt);
    }
}
