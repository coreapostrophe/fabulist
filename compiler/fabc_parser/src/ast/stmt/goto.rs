use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{ast::expr::Expr, error::Error, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct GotoStmt {
    pub target: Box<Expr>,
}

impl Parsable for GotoStmt {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::Goto))?;

        let target = Expr::parse(parser)?;
        parser.consume(TokenKind::Semicolon)?;

        Ok(GotoStmt {
            target: Box::new(target),
        })
    }
}

#[cfg(test)]
mod goto_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{primitive::Primitive, Expr, Primary},
            stmt::goto::GotoStmt,
        },
        Parser,
    };

    #[test]
    fn parses_goto_statements() {
        let source = "goto module_ns.part_ident;";
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize");
        let goto_stmt = Parser::parse::<GotoStmt>(&tokens).expect("Failed to parse goto statement");

        let expected = GotoStmt {
            target: Box::new(Expr::MemberAccess {
                left: Box::new(Expr::Primary(Primary::Primitive(Primitive::Identifier(
                    "module_ns".to_string(),
                )))),
                members: vec![Expr::Primary(Primary::Primitive(Primitive::Identifier(
                    "part_ident".to_string(),
                )))],
            }),
        };

        assert_eq!(goto_stmt, expected);
    }
}
