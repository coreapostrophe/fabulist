use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{ast::expr::Expr, error::Error, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct LetStmt {
    pub name: String,
    pub initializer: Expr,
}

impl Parsable for LetStmt {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        parser.consume(
            Token::Keyword(KeywordKind::Let),
            Error::ExpectedFound(";".to_string(), parser.peek().to_string()),
        )?;

        let name = if let Token::Identifier(ident) = parser.peek() {
            ident.clone()
        } else {
            return Err(Error::ExpectedFound(
                "identifier".to_string(),
                parser.peek().to_string(),
            ));
        };

        parser.advance();

        parser.consume(
            Token::Equal,
            Error::ExpectedFound("=".to_string(), parser.peek().to_string()),
        )?;

        let initializer = parser.expression()?;

        parser.consume(
            Token::Semicolon,
            Error::ExpectedFound(";".to_string(), parser.peek().to_string()),
        )?;

        Ok(LetStmt { name, initializer })
    }
}

#[cfg(test)]
mod let_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{Expr, Primary},
            literal::Literal,
            stmt::r#let::LetStmt,
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_let_statements() {
        let source = "let x = 42;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let stmt = LetStmt::parse(&mut parser).expect("Failed to parse");

        let expected = LetStmt {
            name: "x".to_string(),
            initializer: Expr::Primary(Primary::Literal(Literal::Number(42.0))),
        };

        assert_eq!(stmt, expected);
    }
}
