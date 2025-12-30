use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{ast::stmt::block::BlockStmt, expect_token, Parsable};

#[derive(Debug, PartialEq)]
pub struct FunctionStmt {
    pub id: usize,
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<BlockStmt>,
}

impl Parsable for FunctionStmt {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::Fn))?;

        let name = expect_token!(parser, TokenKind::Identifier, "function name")?;

        let parameters = parser.punctuated(
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::Comma,
            |parser| {
                let param = expect_token!(parser, TokenKind::Identifier, "parameter name")?;
                Ok(param)
            },
        )?;

        let body = Box::new(BlockStmt::parse(parser)?);

        Ok(FunctionStmt {
            id: parser.assign_id(),
            name,
            parameters,
            body,
        })
    }
}

#[cfg(test)]
mod function_stmt_tests {
    use crate::{
        ast::{
            expr::{primitive::Primitive, BinaryOperator, Expr, Primary},
            stmt::{block::BlockStmt, expr::ExprStmt, function::FunctionStmt, Stmt},
        },
        Parser,
    };
    use fabc_lexer::Lexer;

    #[test]
    fn parses_function_stmt() {
        let source = "fn add(a, b) { a + b; }";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");
        let function_stmt =
            Parser::parse::<FunctionStmt>(&tokens).expect("Failed to parse function statement");

        assert_eq!(
            function_stmt,
            FunctionStmt {
                id: 7,
                name: "add".to_string(),
                parameters: vec!["a".to_string(), "b".to_string()],
                body: Box::new(BlockStmt {
                    id: 6,
                    statements: vec![Stmt::Expr(ExprStmt {
                        id: 5,
                        expr: Expr::Binary {
                            id: 4,
                            left: Box::new(Expr::Primary {
                                id: 1,
                                value: Primary::Primitive(Primitive::Identifier {
                                    id: 0,
                                    name: "a".to_string(),
                                },),
                            }),
                            operator: BinaryOperator::Add,
                            right: Box::new(Expr::Primary {
                                id: 3,
                                value: Primary::Primitive(Primitive::Identifier {
                                    id: 2,
                                    name: "b".to_string(),
                                },)
                            }),
                        },
                    })],
                }),
            }
        );
    }
}
