use fabc_error::{Error, Span};
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
        let start_span = parser.start_span();

        parser.consume(TokenKind::Keyword(KeywordKind::Let))?;

        let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;

        parser.consume(TokenKind::Equal)?;

        let initializer = Expr::parse(parser)?;

        parser.consume(TokenKind::Semicolon)?;

        let end_span = parser.end_span();

        Ok(LetStmt {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            name,
            initializer,
        })
    }
}

#[cfg(test)]
mod let_stmt_tests {
    use fabc_error::{LineCol, Span};
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
            info: NodeInfo {
                id: 2,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 11))),
            },
            name: "x".to_string(),
            initializer: Expr::Primary {
                info: NodeInfo {
                    id: 1,
                    span: Span::from((LineCol::new(1, 9), LineCol::new(1, 10))),
                },
                value: Primary::Literal(Literal::Number {
                    info: NodeInfo {
                        id: 0,
                        span: Span::from((LineCol::new(1, 9), LineCol::new(1, 10))),
                    },
                    value: 42.0,
                }),
            },
        };

        assert_eq!(let_stmt, expected);
    }
}
