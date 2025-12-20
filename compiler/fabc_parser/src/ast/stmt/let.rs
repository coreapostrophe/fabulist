use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{ast::expr::Expr, error::Error, expect_token, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct LetStmt {
    pub name: String,
    pub initializer: Expr,
}

impl Parsable for LetStmt {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::Let))?;

        let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;

        parser.consume(TokenKind::Equal)?;

        let initializer = Expr::parse(parser)?;

        parser.consume(TokenKind::Semicolon)?;

        Ok(LetStmt { name, initializer })
    }
}

#[cfg(test)]
mod let_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            stmt::r#let::LetStmt,
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_let_statements() {
        let source = "let x = 42;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(&tokens);
        let stmt = LetStmt::parse(&mut parser).expect("Failed to parse");

        let expected = LetStmt {
            name: "x".to_string(),
            initializer: Expr::Primary(Primary::Literal(Literal::Number(42.0))),
        };

        assert_eq!(stmt, expected);
    }
}
