use fabc_lexer::tokens::TokenKind;

use crate::{ast::expr::Expr, error::Error, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct ExprStmt {
    pub id: usize,
    pub expr: Expr,
}

impl Parsable for ExprStmt {
    fn parse<'src, 'tok>(parser: &mut Parser<'src, 'tok>) -> Result<Self, Error> {
        let expr = Expr::parse(parser)?;
        parser.consume(TokenKind::Semicolon)?;
        Ok(ExprStmt {
            id: parser.assign_id(),
            expr,
        })
    }
}

#[cfg(test)]
mod expr_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, primitive::Primitive, BinaryOperator, Expr, Primary},
            stmt::expr::ExprStmt,
        },
        Parser,
    };

    #[test]
    fn parses_expr_statements() {
        let source = "x + 1;";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");
        let expr_stmt =
            Parser::parse_ast::<ExprStmt>(&tokens).expect("Failed to parse expr statement");

        let expected = ExprStmt {
            id: 4,
            expr: Expr::Binary {
                id: 3,
                left: Box::new(Expr::Primary {
                    id: 1,
                    value: Primary::Primitive(Primitive::Identifier {
                        id: 0,
                        name: "x".to_string(),
                    }),
                }),
                operator: BinaryOperator::Add,
                right: Box::new(Expr::Primary {
                    id: 2,
                    value: Primary::Literal(Literal::Number(1.0)),
                }),
            },
        };
        assert_eq!(expr_stmt, expected);
    }
}
