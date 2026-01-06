use fabc_error::Error;
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{expr::Expr, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct GotoStmt {
    pub info: NodeInfo,
    pub target: Box<Expr>,
}

impl Parsable for GotoStmt {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::Goto))?;

        let target = Expr::parse(parser)?;
        parser.consume(TokenKind::Semicolon)?;

        Ok(GotoStmt {
            info: NodeInfo {
                id: parser.assign_id(),
            },
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
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_goto_statements() {
        let source = "goto module_ns.part_ident;";
        let tokens = Lexer::tokenize(source);
        let goto_stmt =
            Parser::parse_ast::<GotoStmt>(&tokens).expect("Failed to parse goto statement");

        let expected = GotoStmt {
            info: NodeInfo { id: 5 },
            target: Box::new(Expr::MemberAccess {
                info: NodeInfo { id: 4 },
                left: Box::new(Expr::Primary {
                    info: NodeInfo { id: 1 },
                    value: Primary::Primitive(Primitive::Identifier {
                        info: NodeInfo { id: 0 },
                        name: "module_ns".to_string(),
                    }),
                }),
                members: vec![Expr::Primary {
                    info: NodeInfo { id: 3 },
                    value: Primary::Primitive(Primitive::Identifier {
                        info: NodeInfo { id: 2 },
                        name: "part_ident".to_string(),
                    }),
                }],
            }),
        };

        assert_eq!(goto_stmt, expected);
    }
}
