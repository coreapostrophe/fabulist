use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{ast::stmt::block::BlockStmt, error::Error, Parsable};

#[derive(Debug, PartialEq)]
pub struct FunctionStmt {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<BlockStmt>,
}

impl Parsable for FunctionStmt {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(
            Token::Keyword(KeywordKind::Fn),
            Error::ExpectedFound("fn".to_string(), parser.peek().to_string()),
        )?;

        let name = if let Token::Identifier(ident) = parser.advance() {
            ident.to_string()
        } else {
            return Err(Error::ExpectedFound(
                "function name".to_string(),
                parser.peek().to_string(),
            ));
        };

        parser.consume(
            Token::LeftParen,
            Error::ExpectedFound("(".to_string(), parser.peek().to_string()),
        )?;

        let mut parameters = Vec::new();
        if *parser.peek() != Token::RightParen {
            loop {
                if let Token::Identifier(param) = parser.advance() {
                    parameters.push(param.to_string());
                } else {
                    return Err(Error::ExpectedFound(
                        "parameter name".to_string(),
                        parser.peek().to_string(),
                    ));
                }

                if !parser.r#match(vec![Token::Comma]) {
                    break;
                }
            }
        }

        parser.consume(
            Token::RightParen,
            Error::ExpectedFound(")".to_string(), parser.peek().to_string()),
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
            expr::{Expr, Primary},
            primitive::Primitive,
            stmt::{block::BlockStmt, expr::ExprStmt, function::FunctionStmt, Stmt},
        },
        Parsable, Parser,
    };
    use fabc_lexer::{tokens::Token, Lexer};
    #[test]
    fn parse_function_stmt() {
        let source = "fn add(a, b) { a + b; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
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
                            operator: Token::Plus,
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
