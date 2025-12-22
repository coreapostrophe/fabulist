use std::collections::HashMap;

use fabc_lexer::tokens::TokenKind;

use crate::{ast::expr::Expr, expect_token, Parsable};

#[derive(Debug, PartialEq)]
pub struct ObjectDecl {
    pub map: HashMap<String, Expr>,
}

impl Parsable for ObjectDecl {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
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

        Ok(ObjectDecl { map })
    }
}

#[cfg(test)]
mod object_decl_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{ast::decl::object::ObjectDecl, Parsable, Parser};

    #[test]
    fn parses_object_decl() {
        let source = r#"
            {
                key1: "value1",
                key2: 42
            }
        "#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");

        let mut parser = Parser::new(&tokens);
        let object_decl =
            ObjectDecl::parse(&mut parser).expect("Failed to parse object declaration");

        let expected = ObjectDecl {
            map: {
                let mut map = HashMap::new();
                map.insert(
                    "key1".to_string(),
                    crate::ast::expr::Expr::Primary(crate::ast::expr::Primary::Literal(
                        crate::ast::expr::literal::Literal::String("value1".to_string()),
                    )),
                );
                map.insert(
                    "key2".to_string(),
                    crate::ast::expr::Expr::Primary(crate::ast::expr::Primary::Literal(
                        crate::ast::expr::literal::Literal::Number(42.0),
                    )),
                );
                map
            },
        };

        assert_eq!(object_decl, expected);
    }
}
