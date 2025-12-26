use fabc_lexer::tokens::TokenKind;

use crate::{ast::stmt::Stmt, error::Error, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

impl Parsable for BlockStmt {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let mut statements = Vec::new();

        parser.consume(TokenKind::LeftBrace)?;

        while !parser.is_at_end() && parser.peek() != &TokenKind::RightBrace {
            let stmt = Stmt::parse(parser)?;
            statements.push(stmt);
        }

        parser.consume(TokenKind::RightBrace)?;

        Ok(BlockStmt { statements })
    }
}

#[cfg(test)]
mod block_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            stmt::{block::BlockStmt, r#let::LetStmt, Stmt},
        },
        Parser,
    };

    #[test]
    fn parses_block_statements() {
        let source = "{ let a = 1; let b = 2; }";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");
        let block_stmt =
            Parser::parse::<BlockStmt>(&tokens).expect("Failed to parse block statement");

        let expected = BlockStmt {
            statements: vec![
                Stmt::Let(LetStmt {
                    name: "a".to_string(),
                    initializer: Expr::Primary(Primary::Literal(Literal::Number(1.0))),
                }),
                Stmt::Let(LetStmt {
                    name: "b".to_string(),
                    initializer: Expr::Primary(Primary::Literal(Literal::Number(2.0))),
                }),
            ],
        };
        assert_eq!(block_stmt, expected);
    }
}
