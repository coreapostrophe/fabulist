use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{expr::Expr, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct ExprStmt {
    pub info: NodeInfo,
    pub expr: Expr,
}

impl Parsable for ExprStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        let expr = Expr::parse(parser)?;
        parser.consume(TokenKind::Semicolon)?;
        let end_span = parser.end_span();

        Ok(ExprStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            expr,
        })
    }
}

#[cfg(test)]
mod tests {
    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, primitive::Primitive, BinaryOperator, Expr, Primary},
            stmt::expr::ExprStmt,
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_expr_statements() {
        let source = "x + 1;";
        let tokens = Lexer::tokenize(source);
        let expr_stmt =
            Parser::parse_ast::<ExprStmt>(&tokens).expect("Failed to parse expr statement");

        let expected = ExprStmt {
            info: NodeInfo {
                id: 5,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 6))),
            },
            expr: Expr::Binary {
                info: NodeInfo {
                    id: 4,
                    span: Span::from((LineCol::new(1, 1), LineCol::new(1, 5))),
                },
                left: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 1,
                        span: Span::from((LineCol::new(1, 1), LineCol::new(1, 1))),
                    },
                    value: Primary::Primitive(Primitive::Identifier {
                        info: NodeInfo {
                            id: 0,
                            span: Span::from((LineCol::new(1, 1), LineCol::new(1, 1))),
                        },
                        name: "x".to_string(),
                    }),
                }),
                operator: BinaryOperator::Add,
                right: Box::new(Expr::Primary {
                    info: NodeInfo {
                        id: 3,
                        span: Span::from((LineCol::new(1, 5), LineCol::new(1, 5))),
                    },
                    value: Primary::Literal(Literal::Number {
                        info: NodeInfo {
                            id: 2,
                            span: Span::from((LineCol::new(1, 5), LineCol::new(1, 5))),
                        },
                        value: 1.0,
                    }),
                }),
            },
        };
        assert_eq!(expr_stmt, expected);
    }
}
