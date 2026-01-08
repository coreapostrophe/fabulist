use fabc_error::Error;
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct ReturnStmt {
    pub info: NodeInfo,
    pub value: Option<Expr>,
}

impl Parsable for ReturnStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        parser.consume(TokenKind::Keyword(KeywordKind::Return))?;
        let value = if parser.peek() != &TokenKind::Semicolon {
            Some(Expr::parse(parser)?)
        } else {
            None
        };
        parser.consume(TokenKind::Semicolon)?;
        let end_span = parser.end_span();

        Ok(ReturnStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: fabc_error::Span::from((start_span, end_span)),
            },
            value,
        })
    }
}

#[cfg(test)]
mod return_stmt_tests {
    use fabc_error::{LineCol, Span};

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            stmt::r#return::ReturnStmt,
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_return_with_value() {
        let source = "return 42;";
        let return_stmt =
            Parser::parse_ast_str::<ReturnStmt>(source).expect("Failed to parse return stmt");

        assert_eq!(
            return_stmt,
            ReturnStmt {
                info: NodeInfo {
                    id: 1,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 10))),
                },
                value: Some(Expr::Primary {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 8), LineCol::new(1, 9))),
                    },
                    value: Primary::Literal(Literal::Number(42.0)),
                }),
            }
        );
    }

    #[test]
    fn parses_return_without_value() {
        let source = "return;";
        let return_stmt =
            Parser::parse_ast_str::<ReturnStmt>(source).expect("Failed to parse return stmt");

        assert_eq!(
            return_stmt,
            ReturnStmt {
                info: NodeInfo {
                    id: 0,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 7))),
                },
                value: None,
            }
        );
    }
}
