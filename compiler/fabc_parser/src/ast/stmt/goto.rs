use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{ast::expr::Expr, error::Error, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct GotoStmt {
    pub target: Box<Expr>,
}

impl Parsable for GotoStmt {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        parser.consume(Token::Keyword(KeywordKind::Goto))?;

        let target = Expr::parse(parser)?;
        parser.consume(Token::Semicolon)?;

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
        Parsable, Parser,
    };

    #[test]
    fn parses_goto_statements() {
        let source = "goto module_ns.part_ident;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(&tokens);
        let stmt = GotoStmt::parse(&mut parser).expect("Failed to parse");

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

        assert_eq!(stmt, expected);
    }
}
