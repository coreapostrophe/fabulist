use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{ast::stmt::block::BlockStmt, expect_token, Parsable};

#[derive(Debug, PartialEq)]
pub struct FunctionStmt {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<BlockStmt>,
}

impl Parsable for FunctionStmt {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
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
        Parsable, Parser,
    };
    use fabc_lexer::Lexer;

    #[test]
    fn parses_function_stmt() {
        let source = "fn add(a, b) { a + b; }";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");

        let mut parser = Parser::new(&tokens);
        let function_stmt = FunctionStmt::parse(&mut parser).expect("Failed to parse");

        assert_eq!(
            function_stmt,
            FunctionStmt {
                name: "add".to_string(),
                parameters: vec!["a".to_string(), "b".to_string()],
                body: Box::new(BlockStmt {
                    statements: vec![Stmt::Expr(ExprStmt {
                        expr: Expr::Binary {
                            left: Box::new(Expr::Primary(Primary::Primitive(
                                Primitive::Identifier("a".to_string())
                            ))),
                            operator: BinaryOperator::Add,
                            right: Box::new(Expr::Primary(Primary::Primitive(
                                Primitive::Identifier("b".to_string())
                            ))),
                        },
                    })],
                }),
            }
        );
    }
}
