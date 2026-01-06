use std::collections::HashMap;

use fabc_error::Error;
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{expr::Expr, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct ObjectDecl {
    pub info: NodeInfo,
    pub map: HashMap<String, Expr>,
}

impl Parsable for ObjectDecl {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let map_vec = parser.punctuated(
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Comma,
            |parser| {
                let key = expect_token!(parser, TokenKind::Identifier, "identifier")?;
                parser.consume(TokenKind::Colon)?;
                let value = Expr::parse(parser)?;
                Ok((key, value))
            },
        )?;

        let mut map = HashMap::new();
        for (key, value) in map_vec {
            map.insert(key, value);
        }

        Ok(ObjectDecl {
            info: NodeInfo {
                id: parser.assign_id(),
            },
            map,
        })
    }
}

#[cfg(test)]
mod object_decl_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            decl::object::ObjectDecl,
            expr::{literal::Literal, Expr, Primary},
            NodeInfo,
        },
        Parser,
    };

    #[test]
    fn parses_object_decl() {
        let source = r#"
            {
                key1: "value1",
                key2: 42
            }
        "#;
        let tokens = Lexer::tokenize(source);
        let object_decl =
            Parser::parse_ast::<ObjectDecl>(&tokens).expect("Failed to parse object declaration");

        let expected = ObjectDecl {
            info: NodeInfo { id: 2 },
            map: {
                let mut map = HashMap::new();
                map.insert(
                    "key1".to_string(),
                    Expr::Primary {
                        info: NodeInfo { id: 0 },
                        value: Primary::Literal(Literal::String("value1".to_string())),
                    },
                );
                map.insert(
                    "key2".to_string(),
                    Expr::Primary {
                        info: NodeInfo { id: 1 },
                        value: Primary::Literal(Literal::Number(42.0)),
                    },
                );
                map
            },
        };

        assert_eq!(object_decl, expected);
    }
}
