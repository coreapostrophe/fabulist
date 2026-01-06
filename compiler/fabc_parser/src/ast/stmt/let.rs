use fabc_error::Error;
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct LetStmt {
    pub info: NodeInfo,
    pub name: String,
    pub initializer: Expr,
}

impl Parsable for LetStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::Let))?;

        let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;

        parser.consume(TokenKind::Equal)?;

        let initializer = Expr::parse(parser)?;

        parser.consume(TokenKind::Semicolon)?;

        Ok(LetStmt {
            info: NodeInfo {
                id: parser.assign_id(),
            },
            name,
            initializer,
        })
    }
}

#[cfg(test)]
mod let_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            stmt::r#let::LetStmt,
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_let_statements() {
        let source = "let x = 42;";
        let tokens = Lexer::tokenize(source);
        let let_stmt = Parser::parse_ast::<LetStmt>(&tokens).expect("Failed to parse");

        let expected = LetStmt {
            info: NodeInfo { id: 1 },
            name: "x".to_string(),
            initializer: Expr::Primary {
                info: NodeInfo { id: 0 },
                value: Primary::Literal(Literal::Number(42.0)),
            },
        };

        assert_eq!(let_stmt, expected);
    }
}
