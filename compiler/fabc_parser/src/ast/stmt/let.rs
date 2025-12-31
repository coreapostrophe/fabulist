use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{ast::expr::Expr, error::Error, expect_token, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct LetStmt {
    pub id: usize,
    pub name: String,
    pub initializer: Expr,
}

impl Parsable for LetStmt {
    fn parse<'src, 'tok>(parser: &mut Parser<'src, 'tok>) -> Result<Self, Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::Let))?;

        let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;

        parser.consume(TokenKind::Equal)?;

        let initializer = Expr::parse(parser)?;

        parser.consume(TokenKind::Semicolon)?;

        Ok(LetStmt {
            id: parser.assign_id(),
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
        },
        Parser,
    };

    #[test]
    fn parses_let_statements() {
        let source = "let x = 42;";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");
        let let_stmt = Parser::parse_ast::<LetStmt>(&tokens).expect("Failed to parse");

        let expected = LetStmt {
            id: 1,
            name: "x".to_string(),
            initializer: Expr::Primary {
                id: 0,
                value: Primary::Literal(Literal::Number(42.0)),
            },
        };

        assert_eq!(let_stmt, expected);
    }
}
